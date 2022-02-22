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
fn notify_user(title string, str string) {
	os.system('notify-send "$title" "$str" -t 4000')
}

// Hacky way of copying a string to system clipboard
fn clip_str(str string) {
	os.system('echo $str | xclip -sel clip')
}

// Return current time and date
fn datetime() string {
	return time.now().get_fmt_str(.hyphen, .hhmmss24, .ddmmyyyy).replace(' ', '-')
}

// Select screenshot_mode from a list of `items` through `menu`
fn choose_mode(menu []string, items map[string]string) string {
	c := menu.join(' ')
	o := items.keys().join('\n')
	r := os.exec('printf "$o" | $c') or { panic(err) }
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
	opts := maim
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

// Upload image path to imgur
fn upload_to_imgur(path string) http.Response {
	server := {
		'client_id': 'ea6c0ef2987808e'
		'url':       'https://api.imgur.com/3/image'
	}
	file := os.read_file(path) or { panic(err) }
	req := http.FetchConfig{
		method: .post
		headers: {
			'Authorization': 'Client-ID ' + server['client_id']
			'Content-Type':  'image/png'
			'Connection':    'keep-alive'
		}
		data: base64.encode(file)
	}
	res := http.fetch(server['url'], req) or { panic(err) }
	return res
}

// returns imgur url from http.Response
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
	}
}
