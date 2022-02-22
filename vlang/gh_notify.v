module main

import os
import net.http
import json
import time { now, sleep_ms }
import vendor.configparser as cp // dependency
import lib.daemon as d // dependency

/*
Inspired by (basically stolen from) @Conni2461 work :D

Python version:
https://github.com/Conni2461/dotfiles/blob/master/bin/croncmds/github-notify.py

Setup:
	- touch ~/.config/gh-notify.conf
	- copy the following to the gh-notify.conf:
		# [DEFAULT]
		# token = <token>
		# sleep = 60
		# every = 10
		# timeout = 4000

TODOs:
	- TODO: make clicking on the notification popup open the the repo/issue in the browser
	- TODO: Add icons for other notification types.
	- TODO: make clicking on the notification popup open the the repo/issue with octo.nvim
	- TODO: support macos
*/

const (
	configpath = '$os.home_dir()/.config/github-notify.conf'
	github_api = 'https://api.github.com/notifications'
	icons      = {
   'PullRequest': '🔥'
		'Issue':       '📝'
	}
)

struct Repo {
	full_name string
	name      string
	owner     map[string]string
}

struct ResItem {
	id         string
	updated_at string
	repository Repo
	subject    map[string]string
}

struct NotifyMsg {
	icon    string
	title   string
	content string
}

fn config() (string, int, int, int) {
	mut cfg := map[string][]string{}
	cfg['token'] = ['']
	cfg['sleep'] = ['']
	cfg['every'] = ['']
	cfg['timeout'] = ['']

	cp.read_config_section(configpath, 'DEFAULT', mut cfg)

	if cfg['token'][0].len == 0 {
		panic('Token is not found')
	}

	if cfg['sleep'][0].len == 0 {
		cfg['sleep'][0] = '60'
	}

	if cfg['every'][0].len == 0 {
		cfg['every'][0] = '10'
	}

	if cfg['timeout'][0].len == 0 {
		cfg['timeout'][0] = '4000'
	}

	return cfg['token'][0], cfg['every'][0].int(), cfg['sleep'][0].int(), cfg['timeout'][0].int()
}

fn fetch(token string) []ResItem {
	req := http.FetchConfig{
		method: .get
		headers: {
			'Authorization': 'token $token'
		}
	}

	res := http.fetch(github_api, req) or { panic(err) }
	items := json.decode([]ResItem, res.text) or { panic("couldn't parse") }

	return items
}

fn (item ResItem) format() NotifyMsg {
	info := item.repository.full_name.split('/')
	owner, repo := info[0], info[1]
	desc := item.subject['title']
	@type := item.subject['type']
	msg := NotifyMsg{
		icon: icons[@type]
		title: @type
		content: '$owner<b>/$repo</b>\ntitle: <b>$desc</b>'
	}
	return msg
}

fn notify(mut cache map[string]string, items []ResItem, popup_timeout int) {
	for item in items {
		if item.id in cache && cache[item.id] == item.updated_at {
			continue
		} else {
			cache[item.id] = item.updated_at
			msg := item.format()
			args := ['"GH $msg.icon $msg.title"', '"$msg.content"']
			os.system('notify-send ${args.join(' ')} --expire-time=$popup_timeout')
		}
	}
}

fn launch_service() {
	mut cache := map[string]string{}
	token, every, sleep, notify_timeout := config()
	for {
		if now().minute % every == 0 {
			items := fetch(token)
			notify(mut cache, items, notify_timeout)
		}
		sleep_ms(sleep * 1000)
	}
	sleep_ms(sleep * 1000)
}

fn main() {
	d := d.new('/tmp/ghnotify_daemon.pid', launch_service)
	if os.args.len >= 2 {
		match os.args[1] {
			'start' {
				println('starting github notify deamon ...')
				d.start()
			}
			'stop' {
				println('stoping github notify deamon ...')
				d.stop()
			}
			'restart' {
				println('restarting github notify deamon ...')
				d.restart()
			}
			else {
				println('Unknown Command, Supported commands: start, restart, stop')
				exit(2)
			}
		}
		exit(0)
	} else {
		println('usage: ${os.args[0]} start|stop|restart')
		exit(2)
	}
	exit(0)
}
