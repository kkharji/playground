import os
import net.http
import time
import encoding.base64
import json

/*
Imgup: A small utility to take a screenshot then upload it to imgur.
 - 1. select an area, full screen or current window.
 - 2. upload in imgurl.
 - 3. copy the url into clipboard.


req:
dmenu, maim, notify-send and xclip

TODOs:
- TODO: fix fullscreen capturing before the dmenu prompt is closed
- TODO: fix area selection including the mouse.
- TODO: Add support for gif
- TODO: Add support for macos
*/

struct ImgurRes {
	status int
	data   map[string]string
}

// Send notifcation to the user, TODO: add support for macos
fn notify_user(msg_title string, msg_content string) {
	os.system('notify-send "$msg_title" "$msg_content" -t 4000')
}

// Hacky way of copying a string to system clipboard
fn clip_str(text string) {
	os.system('echo $text | xclip -sel clip')
}

// Return current time and date
fn datetime() string {
	return time.now().get_fmt_str(.hyphen, .hhmmss24, .ddmmyyyy).replace(' ', '-')
}

// Select screenshot_mode from a list of `items` through `menu`
fn choose_mode(menu []string, items map[string]string) string {
	c := menu.join(' ')
	o := items.keys().join('\n')

	r := os.execute('printf "$o" | $c')
	if r.exit_code != 0 {
		panic(r.output)
	}
	return r.output.trim_space()
}

// Take a screenshot and return image filepath.
fn take_screenshot() string {
	prmpt := 'capture to imgur'
	dmenu := ['dmenu', '-l', '3', '-i', '-p', '"$prmpt"']
	maim := {
		'current':    'maim -i "$(xdotool getactivewindow)"'
		'area':       'maim -s'
		'fullscreen': 'maim'
	}
	// NOTE: add conditon for current os here
	opts := maim.clone()
	menu := dmenu.clone()
	//
	mode := choose_mode(menu, opts)
	if opts[mode].len != 0 {
		path := '/tmp/${datetime()}.png'
		cmd := opts[mode]
		os.system('$cmd $path')
		return path
	} else {
		return ''
	}
}

// builder error: Header file <openssl/rand.h>, needed for module `net.openssl` was not found. Please install OpenSSL development headers.
// Upload image path to imgur
fn upload_to_imgur(path string) http.Response {
	file := os.read_file(path) or { panic(err) }
	req := http.FetchConfig{
		url: 'https://api.imgur.com/3/image'
		method: .post
		header: http.new_header_from_map({
			.authorization: 'Client-ID ea6c0ef2987808e',
			.content_type:  'image/png',
			.connection:    'keep-alive',
		})
		data: base64.encode(file.bytes())
	}
	res := http.fetch(req) or { panic(err) }
	return res
}

// Returns imgur url from http.Response
fn get_img_url(res http.Response) string {
	r := res.text.str()
	j := json.decode(ImgurRes, r) or { panic(err) }
	return j.data['link']
}

fn main() {
	img_path := take_screenshot()

	if img_path.len != 0 {
		url := get_img_url(upload_to_imgur(img_path))
		clip_str('"![]($url)"')
		// clip_str(get_img_url(upload_to_imgur(img_path)))
		notify_user('IMGUP', 'img link is cliped to clipboard.')
		exit(0)
	}

	exit(1)
}
