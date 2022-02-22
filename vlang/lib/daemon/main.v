module daemon

import os
import time

#include <sys/types.h>
#include <sys/stat.h>
#include <unistd.h>
#include <errno.h>
#include <stdlib.h>
#include <unistd.h>

struct Daemon {
	pidpath string
pub:
	run fn ()
}

fn C.setsid() int

fn C.umask() int

fn C.texit() voidptr

fn C.dup2(oldfd int, newfd int) int

fn (daemon Daemon) daemonize() {
	mut pid := os.fork()
	if pid > 0 {
		exit(0)
	}
	// decouple for parent environment
	os.chdir('/')
	C.setsid()
	C.umask(0)

	// do second fork 
	pid = os.fork()
	if pid > 0 {
		exit(0)
	}
	// redirect standard file descriptors
	C.fflush(C.stdout)
	C.fflush(C.stderr)

	si := os.vfopen('/dev/null', 'r') or { panic(err) }
	so := os.vfopen('/dev/null', 'a+') or { panic(err) }
	se := os.vfopen('/dev/null', 'a+') or { panic(err) }

	C.dup2(os.fileno(si), os.fileno(C.stdin))
	C.dup2(os.fileno(so), os.fileno(C.stdout))
	C.dup2(os.fileno(se), os.fileno(C.stderr))

	pid = os.getpid()
	mut f := os.open_file(daemon.pidpath, 'w+') or { panic(err) }
	defer {
		f.write_str('$pid.str()\n') or { panic(err) }
		f.close()
	}
}

fn (daemon Daemon) getpid() int {
	pid := read_file(daemon.pidpath) or { '0' }
	return pid.int()
}

// start the daemon.
pub fn (daemon Daemon) start() {
	pid := daemon.getpid()
	if pid != 0 {
		eprintln('pidfile $daemon.pidpath already exist. Daemon already running?')
		exit(1)
	}

	daemon.daemonize()
	daemon.run()
	exit(0)
}

// stop the daemon.
pub fn (daemon Daemon) stop() {
	pid := daemon.getpid()
	if pid == 0 {
		eprintln("pidfile $daemon.pidpath doesn\'t exist. Daemon not running?")
		exit(1)
	}
	os.rm(daemon.pidpath) or { }
	for {
		C.kill(pid, 15)
		time.sleep_ms(1 * 1000)
		exit(0)
	}
}

// restart the daemon.
pub fn (daemon Daemon) restart() {
	daemon.stop()
	daemon.start()
}

// new creates a new daemon object
pub fn new(pidpath string, run fn ()) Daemon {
	return Daemon{
		pidpath: pidpath
		run: run
	}
}
