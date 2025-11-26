
mod_writer() {
    local src_dir="${1:-.}"          # 默认当前目录
    local target_file="${2:-./mod.rs}"  # 默认 ./mod.rs
    local mode="${3:-a}"             # 默认 'a'，表示 append

    # 参数检查
    if [[ -z "$src_dir" || -z "$target_file" ]]; then
        echo "Usage: mod_writer <source_dir> <target_file> [a|o]"
        return 1
    fi

    # 检查源目录是否存在
    if [[ ! -d "$src_dir" ]]; then
        echo "Error: source directory '$src_dir' does not exist."
        return 1
    fi

    # 根据模式处理文件
    if [[ "$mode" == "o" ]]; then
        > "$target_file"  # 清空文件
    elif [[ "$mode" != "a" ]]; then
        echo "Error: mode must be 'a' (append) or 'o' (overwrite)"
        return 1
    fi

    # 遍历第一层文件夹
    for dir in "$src_dir"/*(/); do
        folder_name="${dir##*/}"
        echo "pub mod $folder_name;" >> "$target_file"
    done

    echo "Done writing to $target_file"
}

# source /Users/wjj/projects/wjj-rust/src/helper.sh
# mod_writer . lib.rs a

