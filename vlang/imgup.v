/*
Imgup: a small utility to take a screenshot then upload it to imgur.

usage:
- 1. Select mode
- 2. Upload in imgurl.
- 3. Copy the url into clipboard.


req:
- Linux: dmenu, maim, notify-send and xclip.
- Macos: choose-gui, screencapture, pbcopy.


TODOs:
- [X] Support macos
- [ ] Fins tool to do send notificationas
- [ ] Add support for gif
- [ ] Add support for videos
- [ ] Add support for videos
- [ ] fix fullscreen capturing before the dmenu prompt is closed
- [ ] fix interactive selection including the mouse.

MIT License Copyright (C) tami5 2021-2022
*/
import os
import net.http { FetchConfig }
import time
import encoding.base64
import json

struct ImgurRes {
	status int
	data   map[string]string
}

// return current time and date
fn datetime() string {
	return time.now().get_fmt_str(.hyphen, .hhmmss24, .ddmmyyyy).replace(' ', '-')
}

// Hacky way of copying a string to system clipboard
fn set_clipboard(text string) ? {
	clipboard_handler := $if macos { 'pbcopy' } $else { 'xclip -sel clip' }
	result := os.execute('echo $text | $clipboard_handler')

	if result.exit_code != 0 {
		return error('failed to copy $text to clipboard: $result.output')
	}

	println('imgurl is copied')
}

// Send notifcation to the user
// TODO: add support for macos
fn notify_user(msg_content string) {
	$if linux {
		os.system('notify-send "Imgup" "$msg_content" -t 4000')
	}
}

struct UserInterface {
	picker  []string
	command string
	modes   map[string]string
}

fn get_user_interface() ?UserInterface {
	prompt := 'Capture to Imgur'
	$if linux {
		return UserInterface{
			picker: ['dmenu', '-l', '3', '-i', '-p', '"$prompt"']
			command: 'maim'
			modes: {
				'current':   '-i "$(xdotool getactivewindow)"'
				'selection': '-s'
				'full':      ''
			}
		}
	}
	$if macos {
		return UserInterface{
			picker: ['choose', '-n', '4', '-u']
			command: 'screencapture'
			modes: {
				'window':    '-W'
				'current':   "-r -l$(yabai -m query --windows --window | jq '.id')"
				'selection': '-r -i'
				'full':      '-r'
			}
		}
	} $else {
		return error('os is not support')
	}
}

fn (ui UserInterface) get_mode() ?string {
	picker := ui.picker.join(' ')
	choices := ui.modes.keys().join('\n')
	result := os.execute('printf "$choices" | $picker')

	if result.exit_code != 0 {
		error(result.output)
	}

	return result.output.trim_space()
}

fn (ui UserInterface) capture(mode string) ?string {
	args := ui.modes[mode]
	if args == '' {
		return error('Invalid selection')
	}

	path := '/tmp/${datetime()}.png'
	capture := '$ui.command $args $path'
	result := os.execute(capture)

	if result.exit_code != 0 {
		return error('"$capture": $result.output')
	}

	return path
}

fn upload(path string) ?string {
	response := http.fetch(FetchConfig{
		url: 'https://api.imgur.com/3/image'
		method: .post
		data: base64.encode(os.read_bytes(path) ?)
		header: http.new_header_from_map({
			.authorization: 'Client-ID ea6c0ef2987808e',
			.content_type:  'image/png',
			.connection:    'keep-alive',
		})
	}) ?.text.str()

	return json.decode(ImgurRes, response) ?.data['link']
}

fn main() {
	ui := get_user_interface() or {
		println('Unable to get user Inteface: $err')
		exit(1)
	}

	mode := ui.get_mode() or {
		println('Unable to get mode: $err')
		exit(1)
	}

	path := ui.capture(mode) or {
		println('Fail to capture screen: $err')
		exit(1)
	}

	imgurl := upload(path) or {
		println('Unable to get imgur url: $err')
		exit(1)
	}

	set_clipboard('"![]($imgurl)"') or {
		println(err)
		exit(1)
	}

	notify_user('Img link is copied to clipboard.')

	exit(0)
}
