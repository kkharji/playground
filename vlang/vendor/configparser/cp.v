module configparser

// this is the name of the module directory 
import os // needed for handling files 

const (
	kv_separator = `=` // key/values seperator
	mv_separator = `,` // multi-values seperator
	cs_marker    = `#` // comment start marker
	sn_start     = `[` // section name start marker
	sn_end       = `]` // section name end marker
)

// init initializes the module
fn init() {
	// use 'v -d debug *.v' to enable debug mode
	$if debug {
		eprintln('CP: debug mode detected!')
	}
}

// read_file processes the input file
fn read_file(cfg_file_name string) ?[]string { // reads actually the file if found
	$if debug {
		println('CP: opening file $cfg_file_name ...')
	}
	f := os.read_lines(cfg_file_name) or { return error('CP: Failed to open configuration file.') }
	$if debug {
		println('CP: file found and opened sucessfully.')
	}
	return f // returns the string array representing the text file
}

// read_config_section receives the file path, section name and a map containing the keys for tose values will be read 
pub fn read_config_section(cfg_file_name string, section string, mut m map[string][]string) {
	file := read_file(cfg_file_name) or {
		eprintln(err)
		return
	}
	mut in_section := false // reminder if the correct section has been entered
	for line in file { // go through the lines top down
		trimmed_line := line.trim_space() // remove leading and trailing whitespaces
		mut tokenized_line := []string{len: 1} // a parsed line will contain a key part and a values part
		if trimmed_line.len == 0 || trimmed_line[0] == configparser.cs_marker {
			continue
		}
		// skip empty or comment lines, continue with next line
		if tokenize_line(mut tokenized_line, trimmed_line, in_section) == true { // is section begin
			if tokenized_line[0] == configparser.sn_start.str() + section +
				configparser.sn_end.str() { // check if section name matches
				$if debug {
					println('CP: Section $tokenized_line found.')
				}
				in_section = true
			} else { // is another section
				in_section = false
			}
			continue // go to next line
		}
		// if key matches then set the corresponding values
		if tokenized_line[0] in m {
			m[tokenized_line[0]] = tokenized_line[1..]
		}
	}
	$if debug {
		println('CP: Key/Values found: $m')
	}
}

// tokenize_line goes through each line which may contain relevant data and extracts section names, keys and matching values
fn tokenize_line(mut tokenized_line []string, trimmed_line string, in_section bool) bool {
	if trimmed_line[0] == configparser.sn_start { // begin of a section found, return section as key
		tokenized_line[0] = trimmed_line
		return true
	} else if in_section == true { // section entered
		mut val_begin, mut val_end := 0, 0 // initialize bounderies for pulling out the substrings
		for i, character in trimmed_line {
			if character == configparser.cs_marker {
				val_end = i - 1
				tokenized_line << trimmed_line[val_begin..val_end].trim_space()
				break
			}
			if character == configparser.kv_separator {
				val_end = i
				tokenized_line[0] = trimmed_line[val_begin..val_end].trim_space()
				val_begin = i + 1
				continue
			}
			if character == configparser.mv_separator {
				val_end = i
				tokenized_line << trimmed_line[val_begin..val_end].trim_space()
				val_begin = i + 1
				continue
			}
			if i == trimmed_line.len - 1 {
				val_end = i + 1
				tokenized_line << trimmed_line[val_begin..val_end].trim_space()
			}
		}
	}
	return false // this line contained no section begin
}
