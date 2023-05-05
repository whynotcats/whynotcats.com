#!/usr/bin/env nu
# bundle.nu

def main [staging_dir: path, trunk_profile: string] {
    if $trunk_profile == "release" {
        mut stage_dir = $staging_dir

        # If we're on Windows we need to strip some prefix from the path
        if ($stage_dir | str starts-with "\\") {
            print "Cleaning path..."
            $stage_dir = ($stage_dir | str substring (($stage_dir | str index-of ':') - 1)..)
        }

        print "Minifying JS..."
        print $"Looking for JS files in: ($stage_dir)"

        let js_files = (ls ([$stage_dir, '*.js'] | path join) | where type == file | each {|it| $it.name})
        let root_file_names = ($js_files | each {|it| ($it | str substring 0..(($it | str length) - 3)) + '.min.js'})
        let file_table = (($js_files | wrap "name") | merge ($root_file_names | wrap "min"))

        print $"Minifying ($file_table | length) files..."
        $file_table | each {|file| uglifyjs --mangle --compress -- ($file.name) > ($file.min)}
        $file_table | each {|file| mv ($file.min) ($file.name) -f}

        print "Done minifying JS!"
        print "Optimizing wasm..."
        print $"Looking for WASM files in: ($stage_dir)"

        let wasm_files = (ls ([$stage_dir, '*.wasm'] | path join) | where type == file | each {|it| $it.name})
        let root_wasm_names = ($wasm_files | each {|it| ($it | str substring 0..(($it | str length) - 5)) + '_opt.wasm'})
        let wasm_file_table = (($wasm_files | wrap "name") | merge ($root_wasm_names | wrap "opt"))

        print $"Optimizing ($wasm_file_table | length) files..."
        $wasm_file_table | each {|file| wasm-opt ($file.name) -Oz -o ($file.opt)}
        $wasm_file_table | each {|file| mv ($file.opt) ($file.name) -f}
        print "Done optimizing wasm!"

        print "Minifying CSS..."
        print $"Looking for CSS files in: ($stage_dir)"

        let css_files = (ls ([$stage_dir, '*.css'] | path join) | where type == file | each {|it| $it.name})
        let root_css_names = ($css_files | each {|it| ($it | str substring 0..(($it | str length) - 4)) + '.min.css'})
        let css_file_table = (($css_files | wrap "name") | merge ($root_css_names | wrap "min"))

        print $"Optimizing ($css_file_table | length) files..."
        $css_file_table | each {|file| csso ($file.name) --output ($file.min)}
        $css_file_table | each {|file| mv ($file.min) ($file.name) -f}
        print "Done optimizing css!"
    } else {
        print $"Skipping minification... profile: ($trunk_profile)"
    }
}

