#!/bin/bash

# UBS: Urubi's Bash Support v2
# ------------------------------------------------------------------------------

# uncomment to force date stamp in UBS output functions
#UBS_TIMESTAMP="yes"

# uncomment to print executable name in UBS output functions
#UBS_NAMESTAMP="yes"

clrprint(){
    # Special thanks goes to Swashy (I think) and the rest of the arch comunity 
    # for the convienient escape code list at 
    # https://wiki.archlinux.org/index.php/Color_Bash_Prompt
    # Usage clrprint MSG effect1 effect2...
    #   where effects are optional
    local -A color
    local msg
    #declare -A color
    color["none"]='\e[0m';color["Black"]='\e[0;30m';color["Red"]='\e[0;31m'
    color["Green"]='\e[0;32m';color["Yellow"]='\e[0;33m';color["Blue"]='\e[0;34m'
    color["Purple"]='\e[0;35m';color["Cyan"]='\e[0;36m';color["White"]='\e[0;37m'      
    color["Bold Black"]='\e[1;30m';color["Bold Red"]='\e[1;31m';color["Bold Green"]='\e[1;32m'      
    color["Bold Yellow"]='\e[1;33m';color["Bold Blue"]='\e[1;34m';color["Bold Purple"]='\e[1;35m'     
    color["Bold Cyan"]='\e[1;36m';color["Bold White"]='\e[1;37m';color["Underlined Black"]='\e[4;30m'
    color["Underlined Red"]='\e[4;31m';color["Underlined Green"]='\e[4;32m'
    color["Underlined Yellow"]='\e[4;33m';color["Underlined Blue"]='\e[4;34m'    
    color["Underlined Purple"]='\e[4;35m';color["Underlined Cyan"]='\e[4;36m'    
    color["Underlined White"]='\e[4;37m';color["Background Black"]='\e[40m'     
    color["Background Red"]='\e[41m';color["Background Green"]='\e[42m'     
    color["Background Yellow"]='\e[43m';color["Background Blue"]='\e[44m'      
    color["Background Purple"]='\e[45m';color["Background Cyan"]='\e[46m'      
    color["Background White"]='\e[47m';color["Intense Black"]='\e[0;90m'      
    color["Intense Red"]='\e[0;91m';color["Intense Green"]='\e[0;92m'      
    color["Intense Yellow"]='\e[0;93m';color["Intense Blue"]='\e[0;94m'       
    color["Intense Purple"]='\e[0;95m';color["Intense Cyan"]='\e[0;96m'       
    color["Intense White"]='\e[0;97m';color["Intense Bold Black"]='\e[1;90m' 
    color["Intense Bold Red"]='\e[1;91m';color["Intense Bold Green"]='\e[1;92m' 
    color["Intense Bold Yellow"]='\e[1;93m';color["Intense Bold Blue"]='\e[1;94m'  
    color["Intense Bold Purple"]='\e[1;95m';color["Intense Bold Cyan"]='\e[1;96m'  
    color["Intense Bold White"]='\e[1;97m';color["Intense Background Black"]='\e[0;100m' 
    color["Intense Background Red"]='\e[0;101m';color["Intense Background Green"]='\e[0;102m' 
    color["Intense Background Yellow"]='\e[0;103m';color["Intense Background Blue"]='\e[0;104m'  
    color["Intense Background Purple"]='\e[0;105m';color["Intense Background Cyan"]='\e[0;106m'
    color["Intense Background White"]='\e[0;107m'
    msg="$1"; shift
    [[ -t 1 ]] && {
        for i in "$@"; do printf "${color["$i"]}"; done
        printf "$msg"${color["none"]} 
    } || printf "$1"
}

# rich output facilities
line_header(){
    local desc ds n
    [[ "$1" ]] && desc="$1: " || desc=""
    [[ "$UBS_TIMESTAMP" ]] && ds="$(date +'%Y/%m/%d %H:%M:%S') " || ds=""
    [[ "$UBS_NAMESTAMP" ]] && n="$(basename "$0") " || n=""
    echo "$n$ds$desc"
}
die(){      echo ""; clrprint "$(line_header FATAL)$1\n" "Intense Bold Red"; exit 1; }
warn(){     clrprint "$(line_header Warning)$1\n" "Yellow"; }
shout(){    clrprint "$(line_header)$1\n" "Intense Bold White"; }
print(){    clrprint "$(line_header)$1\n"; }
debug(){ 
    [[ "$DEBUG" ]] && clrprint "$(line_header DEBUG) $1\n"; 
}
ok(){
    local m
    [[ "$1" ]] && m="$1" || m='[ok]'
    clrprint "$(line_header)$m\n" "Intense Green"
    exit 0
}

require(){
    # require: assert required binaries or interface equvilants are present 
    # on the system. prints the name of the binary if found.
    # Usage: require binary binary_equvilant binary_equvilant_2
    for i in "$@"; do 
        hash "$i" 2>/dev/null && {
            echo "$i"
            return
        }
    done
    die "at least one of the following programs is required, but none were found on this system: $@"
}

require_files(){
    # require: assert required files are present in the current directory
    # Usage: require_files file1 file2 ...
    for i in "$@"; do 
        test -f "$i" 2>/dev/null || die "'$i' was not found in the current directory"
    done
}


mute() {
    "$@" >/dev/null 2>&1
}
# ------------------------------------------------------------------------------

[[ "$1" == "test"  ]] && {
    for i in base/ulc*; do
        echo -n "Testing $(basename $i)...          "
        (cd "$i" && cargo test) || die "testing failed for $(basename $i)"
        echo "Done"
    done
}

[[ "$1" == "clean" ]] && {
    for i in base/ulc*; do
        echo -n "Cleaning temp files in $(basename $i)...          "
        (cd "$i" && cargo clean) || die "Something wierd happend!!"
        echo "Done"
    done

}

[[ "$1" == "publish" ]] && {
    $0 clean || die "couldn't clean module targets"
    git commit -a || die "couldn't commit changes"
    torify git push -u origin master || die "couldn't update remote"
}

ok
