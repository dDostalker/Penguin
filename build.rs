use std::env;
use std::fs;
use std::path::Path;

fn main() {
    println!("cargo:rerun-if-changed=config/language.toml");
    let config_path = "config/language.toml";
    let config = match load_or_create_config(config_path) {
        Ok(config) => config,
        Err(e) => {
            eprintln!("Error loading config: {}", e);
            eprintln!("Using default Chinese configuration");
            create_default_config()
        }
    };

    let language = config["language"].as_str().unwrap_or("chinese");

    let lang_config = if let Some(lang_cfg) = config.get(language) {
        lang_cfg
    } else {
        eprintln!(
            "Warning: Language '{}' not found in config, falling back to chinese",
            language
        );
        if let Some(chinese_cfg) = config.get("chinese") {
            chinese_cfg
        } else {
            eprintln!("Error: Chinese configuration not found, using hardcoded defaults");
            &create_default_config()["chinese"]
        }
    };

    let constants = generate_constants(language, lang_config);

    let out_dir = env::var("OUT_DIR").expect("OUT_DIR not set");
    let output_path = format!("{}/language_constants.rs", out_dir);

    if let Err(e) = fs::write(&output_path, constants) {
        eprintln!("Error writing constants file: {}", e);
        std::process::exit(1);
    }

    println!("cargo:rustc-env=CURRENT_LANGUAGE={}", language);
}

fn load_or_create_config(config_path: &str) -> Result<toml::Value, Box<dyn std::error::Error>> {
    if !Path::new(config_path).exists() {
        if let Err(e) = fs::create_dir_all("config") {
            return Err(format!("Failed to create config directory: {}", e).into());
        }

        let default_config = create_default_config_string();
        if let Err(e) = fs::write(config_path, default_config) {
            return Err(format!("Failed to write default config: {}", e).into());
        }
    }

    let config_content = fs::read_to_string(config_path)?;
    let config: toml::Value = toml::from_str(&config_content)?;

    Ok(config)
}

fn create_default_config_string() -> String {
    r#"# 语言配置文件
# 可选项: chinese, english
language = "chinese"

[chinese]
app_title = "Penguin PE 分析器"
virtual_address_to_file_offset = "虚拟地址->文件偏移"
virtual_address_label = "虚拟地址 (支持10进制和16进制，如: 1234 或 0x4D2):"
file_offset_label = "文件偏移:"
not_found = "未找到"
close_button = "关闭"
about_title = "关于 Penguin"
version = "版本: 0.1.0"
author = "作者: dDostalker"
Pedescription = "描述: 一个强大的PE文件分析工具"
ok_button = "确定"
settings_title = "设置"
app_settings = "应用程序设置"
demo_notifications = "演示通知"
cancel_button = "取消"
help_title = "帮助"
usage_help = "使用帮助"

# 菜单项
tools_menu = "工具"
settings_menu = "设置"
virtual_address_to_file_offset_menu = "虚拟地址->文件偏移"
export_menu = "导出为..."
help_menu = "帮助"
usage_help_menu = "使用帮助"
about_menu = "关于"

# 导入表相关
dll_list = "DLL列表"
function_list = "函数列表"
select_dll_prompt = "请选择一个DLL查看其函数"
dll_name = "DLL名称"
function_count = "函数数量"
operation = "操作"
select_button = "选择"
open_location = "打开位置"
function_name = "函数名"
function_details = "导出函数详情"
sequence_number = "序号"
detail_button = "详细"

# 消息提示
save_success = "保存成功"
backup_success = "备份成功"
import_table_modified = "修改导入表成功"
export_table_modified = "修改导出表成功"
file_not_found = "文件不存在"
save_failed = "保存失败"
file_handle_closed = "文件句柄已关闭"

# 文件菜单
file_menu = "文件"
open_button = "打开"
exit_button = "退出"
save_button = "保存"
save_as_format = "保存为{}"

            # 错误消息
            cannot_extract_filename = "无法提取文件名"
            not_normal_machine_image = "不是正常的机器映像文件"
            hex_parse_error = "16进制解析错误: {}"
            decimal_parse_error = "10进制解析错误: {}"
                        cannot_get_file_directory = "无法获取文件所在目录: {}"

            # 序列化相关错误消息
            serialize_toml_failed = "序列化TOML失败: {}"
            serialize_json_failed = "序列化JSON失败: {}"
            unsupported_file_type = "不支持的文件类型: {}"
            serialize_failed = "序列化失败: {}"
            save_failed_error = "保存失败: {}"

            # PE文件相关错误消息
            not_valid_pe_file = "打开文件不是有效的PE文件"

            # 节表相关
no_sections = "该文件无节表"
section_name = "节名称"
virtual_address = "虚拟地址"
size = "大小"
file_offset = "文件偏移"
relocation_address = "重定位地址"
characteristics = "特征"
copy_button = "复制"
section_info_format = "节名: {}\n虚拟地址: {}\n大小: {}\n文件偏移: {}\n特征: {}"

# DOS Stub相关
no_dos_stub = "该文件无 DOS Stub 数据"
offset = "偏移"
hexadecimal = "十六进制"

# NT头相关
field_name = "字段名"
value = "值"
description = "描述"
file_header_signature = "文件头签名"
machine_description = "标记可以程序可以运行在什么样的CPU上"
number_of_sections = "节数"
timestamp = "时间戳"
pointer_to_symbol_table = "符号表指针"
number_of_symbols = "符号数"
size_of_optional_header = "可选头大小"
characteristics_label = "文件属性"
magic = "标记文件头"
major_linker_version = "链接器主版本号"
minor_linker_version = "链接器次版本号"
size_of_code = "代码大小"
size_of_initialized_data = "初始化数据大小"
size_of_uninitialized_data = "未初始化数据大小"
address_of_entry_point = "入口点地址"
base_of_code = "代码基址"
base_of_data = "数据基址"
image_base = "映像基址"
section_alignment = "节对齐"
file_alignment = "文件对齐"
major_operating_system_version = "操作系统主版本号"
minor_operating_system_version = "操作系统次版本号"
major_image_version = "映像主版本号"
minor_image_version = "映像次版本号"
major_subsystem_version = "子系统主版本号"
minor_subsystem_version = "子系统次版本号"
win32_version_value = "Win32版本值"
size_of_image = "映像大小"
size_of_headers = "头大小"
checksum = "校验和"
subsystem = "子系统"
dll_characteristics = "DLL特征"
size_of_stack_reserve = "栈保留大小"
size_of_stack_commit = "栈提交大小"
size_of_heap_reserve = "堆保留大小"
size_of_heap_commit = "堆提交大小"
loader_flags = "加载器标志"
number_of_rva_and_sizes = "RVA和大小数量"

# DOS头相关
dos_header_title = "DOS Header"
dos_header_field_name = "字段名"
dos_header_value = "值"
dos_header_description = "描述"
dos_header_e_magic = "DOS签名"
dos_header_e_cblp = "文件中的全部和部分页数"
dos_header_e_cp = "文件中的全部和部分页数"
dos_header_e_crlc = "重定位表中的指针数"
dos_header_e_cparhdr = "头部尺寸以段落为单位"
dos_header_e_minalloc = "所需最小附件段"
dos_header_e_maxalloc = "所需最大附件段"
dos_header_e_ss = "初始堆栈段"
dos_header_e_sp = "初始堆栈指针"
dos_header_e_csum = "文件校验和"
dos_header_e_ip = "入口点段"
dos_header_e_cs = "入口点段段"
dos_header_e_lfarlc = "重定位表段"
dos_header_e_ovno = "覆盖号"
dos_header_e_res = "保留字段"
dos_header_e_oemid = "OEM标识符"
dos_header_e_oeminfo = "OEM信息"
dos_header_e_res2 = "保留字段2"
dos_header_e_lfanew = "PE头偏移"

# 导出表相关
export_table_title = "Export Table"
export_function_name = "函数名"
export_function_virtual_address = "函数虚拟地址"
export_operation = "操作"
export_detail_button = "详情"
export_function_details = "导出函数详情"

# 导出表编辑相关
target_virtual_address = "目标虚拟地址:"
address_updated = "地址已更新"
invalid_hex_address_format = "无效的十六进制地址格式"

# 演示消息
demo_operation_success = "操作成功完成！"
demo_error_occurred = "发生了一个错误"
demo_warning_notice = "请注意这个警告"
demo_info_message = "这是一条信息提示"

# 计算相关错误消息
calc_md5_failed = "计算md5失败"
calc_sha1_failed = "计算sha1失败"

# NT头字段描述
nt_header_file_characteristics = "文件属性"
nt_header_magic = "标记文件头"
# 数据目录相关
data_directory_name = "目录名称"
nt_header_major_linker_version = "链接器主版本号"
nt_header_minor_linker_version = "链接器次版本号"
nt_header_size_of_code = "代码大小"
nt_header_size_of_initialized_data = "初始化数据大小"
nt_header_size_of_uninitialized_data = "未初始化数据大小"
nt_header_address_of_entry_point = "入口点地址"
nt_header_base_of_code = "代码基址"
nt_header_base_of_data = "数据基址"
nt_header_image_base = "映像基址"
nt_header_section_alignment = "节对齐"
nt_header_file_alignment = "文件对齐"
nt_header_major_operating_system_version = "操作系统主版本号"
nt_header_minor_operating_system_version = "操作系统次版本号"
nt_header_major_image_version = "映像主版本号"
nt_header_minor_image_version = "映像次版本号"
nt_header_major_subsystem_version = "子系统主版本号"
nt_header_minor_subsystem_version = "子系统次版本号"
nt_header_win32_version_value = "Win32版本值"
nt_header_size_of_image = "映像大小"
nt_header_size_of_headers = "头大小"
nt_header_checksum = "校验和"
nt_header_subsystem = "子系统"
nt_header_dll_characteristics = "DLL属性"
nt_header_size_of_stack_reserve = "堆栈预留大小"
nt_header_size_of_stack_commit = "堆栈提交大小"
nt_header_size_of_heap_reserve = "堆预留大小"
nt_header_size_of_heap_commit = "堆提交大小"
nt_header_loader_flags = "加载器属性"
nt_header_number_of_rva_and_sizes = "RVA和尺寸数"

# 机器架构描述
machine_x86_32 = "32位x86架构"
machine_mips_big_endian = "MIPS大端"
machine_mips_little_endian = "MIPS小端"
machine_alpha = "Alpha"
machine_sh3_little_endian = "SH3小端"
machine_sh3e_little_endian = "SH3E小端"
machine_sh4_little_endian = "SH4小端"
machine_sh5 = "SH5"
machine_arm_little_endian = "ARM小端"
machine_arm_thumb_little_endian = "ARM Thumb/Thumb-2 小端"
machine_arm = "ARM"
machine_ibm = "IBM"
machine_powercfp = "POWERCFP"
machine_intel_64 = "Intel 64"
machine_mips = "MIPS"
machine_alpha64 = "ALPHA64"
machine_infineon = "Infineon"
machine_x64_64 = "64位x64架构"
machine_arm64_little_endian = "ARM64 小端"
machine_unknown = "unknown"

# 文件特征描述
characteristics_relocs_stripped = "重定位信息被剥离"
characteristics_executable_image = "文件是可执行的"
characteristics_line_nums_stripped = "行号被剥离"
characteristics_local_syms_stripped = "本地符号被剥离"
characteristics_aggressive_ws_trim = "积极地修剪工作集"
characteristics_large_address_aware = "应用程序可以处理>2gb地址"
characteristics_bytes_reversed_lo = "机器字节是反向的"
characteristics_32bit_machine = "32位机器字"
characteristics_debug_stripped = "调试信息被剥离"
characteristics_removable_run_from_swap = "如果映像在可移动媒体上，则从交换文件中复制并运行"
characteristics_net_run_from_swap = "如果映像在网络上，则从交换文件中复制并运行"
characteristics_system = "系统文件"
characteristics_dll = "文件是DLL"
characteristics_up_system_only = "文件应该只在UP机器上运行"
characteristics_bytes_reversed_hi = "机器字节是反向的"

# DLL特征描述
dll_characteristics_appcontainer = "映像必须在AppContainer中运行"
dll_characteristics_control_flow_guard = "控制流保护"
dll_characteristics_dynamic_base = "DLL可重定位"
dll_characteristics_force_integrity = "强制实施代码完整性检查"
dll_characteristics_high_entropy_va = "映像可以处理64位高熵VA空间"
dll_characteristics_nobind = "禁止绑定"
dll_characteristics_nolsolation = "映像理解隔离但不隔离"
dll_characteristics_noseh = "不使用SEH，不能处理任何有SE的处理程序"
dll_characteristics_nxcompat = "NX兼容"
dll_characteristics_processinit = "进程初始化"
dll_characteristics_processterm = "进程终止"
dll_characteristics_terminalserveraware = "终端服务器感知"
dll_characteristics_threadinit = "线程初始化"
dll_characteristics_threadterm = "线程终止"
dll_characteristics_wdmdriver = "WDM驱动程序"

# 节特征描述
section_reserved = "保留"
section_no_pad = "不应将节填充到下一个边界。此标志已过时，由 IMAGE_SCN_ALIGN_1BYTES 取代"
section_contains_code = "节包含可执行代码"
section_contains_initialized_data = "节包含初始化数据"
section_contains_uninitialized_data = "节包含未初始化数据"
section_other = "保留"
section_info = "节包含注释或其他信息。它仅对对象文件有效"
section_remove = "节不会成为映像的一部分。它仅对对象文件有效"
section_comdat = "节包含 COMDAT 数据。它仅对对象文件有效"
section_no_defer_spec_exc = "重置本部分的 TLB 条目中处理位的推理异常"
section_gprel = "节包含通过全局指针引用的数据"
section_align_1bytes = "在 1 字节边界上对齐数据。它仅对对象文件有效"
section_align_2bytes = "在 2 字节边界上对齐数据。它仅对对象文件有效"
section_align_4bytes = "在 4 字节边界上对齐数据。它仅对对象文件有效"
section_align_8bytes = "对齐 8 字节边界上的数据。它仅对对象文件有效"
section_align_16bytes = "在 16 字节边界上对齐数据。它仅对对象文件有效"
section_align_32bytes = "在 32 字节边界上对齐数据。它仅对对象文件有效"
section_align_64bytes = "在 64 字节边界上对齐数据。它仅对对象文件有效"
section_align_128bytes = "在 128 字节边界上对齐数据。它仅对对象文件有效"
section_align_256bytes = "在 256 字节边界上对齐数据。它仅对对象文件有效"
section_align_512bytes = "在 512 字节边界上对齐数据。它仅对对象文件有效"
section_align_1024bytes = "在 1024 字节边界上对齐数据。它仅对对象文件有效"
section_align_2048bytes = "在 2048 字节边界上对齐数据。它仅对对象文件有效"
section_align_4096bytes = "在 4096 字节边界上对齐数据。它仅对对象文件有效"
section_align_8192bytes = "对齐 8192 字节边界上的数据。它仅对对象文件有效"
section_reloc_ovfl = "节包含扩展重定位。节的重定位计数超过了节标头中为其保留的 16 位"
section_mem_discardable = "可以根据需要丢弃节"
section_mem_not_cached = "无法缓存节"
section_mem_not_paged = "该节不能分页"
section_mem_shared = "可以在内存中共享节"
section_mem_execute = "节可以作为代码执行"
section_mem_read = "可以读取节"
section_mem_write = "可以写入节"

# DLL调试相关
function_not_found = "函数不存在"

[english]
app_title = "Penguin PE Analyzer"
virtual_address_to_file_offset = "Virtual Address -> File Offset"
virtual_address_label = "Virtual Address (supports decimal and hex, e.g.: 1234 or 0x4D2):"
file_offset_label = "File Offset:"
not_found = "Not Found"
close_button = "Close"
about_title = "About Penguin"
version = "Version: 0.1.0"
author = "Author: dDostalker"
Pedescription = "Description: A powerful PE file analysis tool"
ok_button = "OK"
settings_title = "Settings"
app_settings = "Application Settings"
demo_notifications = "Demo Notifications"
cancel_button = "Cancel"
help_title = "Help"
usage_help = "Usage Help"

# 菜单项
tools_menu = "Tools"
settings_menu = "Settings"
virtual_address_to_file_offset_menu = "Virtual Address -> File Offset"
export_menu = "Export As..."
help_menu = "Help"
usage_help_menu = "Usage Help"
about_menu = "About"

# 导入表相关
dll_list = "DLL List"
function_list = "Function List"
select_dll_prompt = "Please select a DLL to view its functions"
dll_name = "DLL Name"
function_count = "Function Count"
operation = "Operation"
select_button = "Select"
open_location = "Open Location"
function_name = "Function Name"
function_details = "Export Function Details"
sequence_number = "Sequence Number"
detail_button = "Details"

# 消息提示
save_success = "Save successful"
backup_success = "Backup successful"
import_table_modified = "Import table modified successfully"
export_table_modified = "Export table modified successfully"
file_not_found = "File not found"
save_failed = "Save failed"
file_handle_closed = "File handle closed"

# 文件菜单
file_menu = "File"
open_button = "Open"
exit_button = "Exit"
save_button = "Save"
save_as_format = "Save as {}"

            # 错误消息
            cannot_extract_filename = "Cannot extract filename"
            not_normal_machine_image = "Not a normal machine image file"
            hex_parse_error = "Hex parse error: {}"
            decimal_parse_error = "Decimal parse error: {}"
                        cannot_get_file_directory = "Cannot get file directory: {}"

            # 序列化相关错误消息
            serialize_toml_failed = "Serialize TOML failed: {}"
            serialize_json_failed = "Serialize JSON failed: {}"
            unsupported_file_type = "Unsupported file type: {}"
            serialize_failed = "Serialize failed: {}"
            save_failed_error = "Save failed: {}"

            # PE文件相关错误消息
            not_valid_pe_file = "Opened file is not a valid PE file"

            # 节表相关
no_sections = "This file has no sections"
section_name = "Section Name"
virtual_address = "Virtual Address"
size = "Size"
file_offset = "File Offset"
relocation_address = "Relocation Address"
characteristics = "Characteristics"
copy_button = "Copy"
section_info_format = "Section Name: {}\nVirtual Address: {}\nSize: {}\nFile Offset: {}\nCharacteristics: {}"

# DOS Stub相关
no_dos_stub = "This file has no DOS Stub data"
offset = "Offset"
hexadecimal = "Hexadecimal"

            # NT头相关
            field_name = "Field Name"
            value = "Value"
            description = "Description"
            # 数据目录相关
            data_directory_name = "Directory Name"
file_header_signature = "File Header Signature"
machine_description = "Indicates the type of CPU the program can run on"
number_of_sections = "Number of Sections"
timestamp = "Timestamp"
pointer_to_symbol_table = "Pointer to Symbol Table"
number_of_symbols = "Number of Symbols"
size_of_optional_header = "Size of Optional Header"
characteristics_label = "File Characteristics"
magic = "Magic"
major_linker_version = "Major Linker Version"
minor_linker_version = "Minor Linker Version"
size_of_code = "Size of Code"
size_of_initialized_data = "Size of Initialized Data"
size_of_uninitialized_data = "Size of Uninitialized Data"
address_of_entry_point = "Address of Entry Point"
base_of_code = "Base of Code"
base_of_data = "Base of Data"
image_base = "Image Base"
section_alignment = "Section Alignment"
file_alignment = "File Alignment"
major_operating_system_version = "Major Operating System Version"
minor_operating_system_version = "Minor Operating System Version"
major_image_version = "Major Image Version"
minor_image_version = "Minor Image Version"
major_subsystem_version = "Major Subsystem Version"
minor_subsystem_version = "Minor Subsystem Version"
win32_version_value = "Win32 Version Value"
size_of_image = "Size of Image"
size_of_headers = "Size of Headers"
checksum = "Checksum"
subsystem = "Subsystem"
dll_characteristics = "DLL Characteristics"
size_of_stack_reserve = "Size of Stack Reserve"
size_of_stack_commit = "Size of Stack Commit"
size_of_heap_reserve = "Size of Heap Reserve"
size_of_heap_commit = "Size of Heap Commit"
loader_flags = "Loader Flags"
number_of_rva_and_sizes = "Number of RVA and Sizes"

# DOS头相关
dos_header_title = "DOS Header"
dos_header_field_name = "Field Name"
dos_header_value = "Value"
dos_header_description = "Description"
dos_header_e_magic = "DOS Signature"
dos_header_e_cblp = "Number of pages in file"
dos_header_e_cp = "Number of pages in file"
dos_header_e_crlc = "Number of relocations"
dos_header_e_cparhdr = "Size of header in paragraphs"
dos_header_e_minalloc = "Minimum extra paragraphs needed"
dos_header_e_maxalloc = "Maximum extra paragraphs needed"
dos_header_e_ss = "Initial (relative) SS value"
dos_header_e_sp = "Initial SP value"
dos_header_e_csum = "Checksum"
dos_header_e_ip = "Initial IP value"
dos_header_e_cs = "Initial (relative) CS value"
dos_header_e_lfarlc = "File address of relocation table"
dos_header_e_ovno = "Overlay number"
dos_header_e_res = "Reserved words"
dos_header_e_oemid = "OEM identifier"
dos_header_e_oeminfo = "OEM information"
dos_header_e_res2 = "Reserved words 2"
dos_header_e_lfanew = "File address of new exe header"

# 导出表相关
export_table_title = "Export Table"
export_function_name = "Function Name"
export_function_virtual_address = "Function Virtual Address"
export_operation = "Operation"
export_detail_button = "Details"
export_function_details = "Export Function Details"

# 导出表编辑相关
target_virtual_address = "Target Virtual Address:"
address_updated = "Address updated"
invalid_hex_address_format = "Invalid hexadecimal address format"

# 演示消息
demo_operation_success = "Operation completed successfully!"
demo_error_occurred = "An error occurred"
demo_warning_notice = "Please note this warning"
demo_info_message = "This is an information message"

# 计算相关错误消息
calc_md5_failed = "Failed to calculate MD5"
calc_sha1_failed = "Failed to calculate SHA1"

# NT头字段描述
nt_header_file_characteristics = "File Characteristics"
nt_header_magic = "Magic"
nt_header_major_linker_version = "Major Linker Version"
nt_header_minor_linker_version = "Minor Linker Version"
nt_header_size_of_code = "Size of Code"
nt_header_size_of_initialized_data = "Size of Initialized Data"
nt_header_size_of_uninitialized_data = "Size of Uninitialized Data"
nt_header_address_of_entry_point = "Address of Entry Point"
nt_header_base_of_code = "Base of Code"
nt_header_base_of_data = "Base of Data"
nt_header_image_base = "Image Base"
nt_header_section_alignment = "Section Alignment"
nt_header_file_alignment = "File Alignment"
nt_header_major_operating_system_version = "Major Operating System Version"
nt_header_minor_operating_system_version = "Minor Operating System Version"
nt_header_major_image_version = "Major Image Version"
nt_header_minor_image_version = "Minor Image Version"
nt_header_major_subsystem_version = "Major Subsystem Version"
nt_header_minor_subsystem_version = "Minor Subsystem Version"
nt_header_win32_version_value = "Win32 Version Value"
nt_header_size_of_image = "Size of Image"
nt_header_size_of_headers = "Size of Headers"
nt_header_checksum = "Checksum"
nt_header_subsystem = "Subsystem"
nt_header_dll_characteristics = "DLL Characteristics"
nt_header_size_of_stack_reserve = "Size of Stack Reserve"
nt_header_size_of_stack_commit = "Size of Stack Commit"
nt_header_size_of_heap_reserve = "Size of Heap Reserve"
nt_header_size_of_heap_commit = "Size of Heap Commit"
nt_header_loader_flags = "Loader Flags"
nt_header_number_of_rva_and_sizes = "Number of RVA and Sizes"

# Machine architecture descriptions
machine_x86_32 = "32-bit x86 architecture"
machine_mips_big_endian = "MIPS big-endian"
machine_mips_little_endian = "MIPS little-endian"
machine_alpha = "Alpha"
machine_sh3_little_endian = "SH3 little-endian"
machine_sh3e_little_endian = "SH3E little-endian"
machine_sh4_little_endian = "SH4 little-endian"
machine_sh5 = "SH5"
machine_arm_little_endian = "ARM little-endian"
machine_arm_thumb_little_endian = "ARM Thumb/Thumb-2 little-endian"
machine_arm = "ARM"
machine_ibm = "IBM"
machine_powercfp = "POWERCFP"
machine_intel_64 = "Intel 64"
machine_mips = "MIPS"
machine_alpha64 = "ALPHA64"
machine_infineon = "Infineon"
machine_x64_64 = "64-bit x64 architecture"
machine_arm64_little_endian = "ARM64 little-endian"
machine_unknown = "unknown"

# File characteristics descriptions
characteristics_relocs_stripped = "Relocation information stripped"
characteristics_executable_image = "File is executable"
characteristics_line_nums_stripped = "Line numbers stripped"
characteristics_local_syms_stripped = "Local symbols stripped"
characteristics_aggressive_ws_trim = "Aggressively trim working set"
characteristics_large_address_aware = "Application can handle >2GB addresses"
characteristics_bytes_reversed_lo = "Machine bytes are reversed"
characteristics_32bit_machine = "32-bit machine word"
characteristics_debug_stripped = "Debug information stripped"
characteristics_removable_run_from_swap = "If image is on removable media, copy and run from swap file"
characteristics_net_run_from_swap = "If image is on network, copy and run from swap file"
characteristics_system = "System file"
characteristics_dll = "File is DLL"
characteristics_up_system_only = "File should only run on UP machine"
characteristics_bytes_reversed_hi = "Machine bytes are reversed"

# DLL characteristics descriptions
dll_characteristics_appcontainer = "Image must run in AppContainer"
dll_characteristics_control_flow_guard = "Control Flow Guard"
dll_characteristics_dynamic_base = "DLL can be relocated"
dll_characteristics_force_integrity = "Force code integrity checks"
dll_characteristics_high_entropy_va = "Image can handle 64-bit high entropy VA space"
dll_characteristics_nobind = "Do not bind"
dll_characteristics_nolsolation = "Image understands isolation but doesn't isolate"
dll_characteristics_noseh = "No SEH, cannot handle any SE handlers"
dll_characteristics_nxcompat = "NX compatible"
dll_characteristics_processinit = "Process initialization"
dll_characteristics_processterm = "Process termination"
dll_characteristics_terminalserveraware = "Terminal server aware"
dll_characteristics_threadinit = "Thread initialization"
dll_characteristics_threadterm = "Thread termination"
dll_characteristics_wdmdriver = "WDM driver"

# Section characteristics descriptions
section_reserved = "Reserved"
section_no_pad = "Section should not be padded to next boundary. This flag is obsolete and replaced by IMAGE_SCN_ALIGN_1BYTES"
section_contains_code = "Section contains executable code"
section_contains_initialized_data = "Section contains initialized data"
section_contains_uninitialized_data = "Section contains uninitialized data"
section_other = "Reserved"
section_info = "Section contains comments or other information. Valid only for object files"
section_remove = "Section will not become part of image. Valid only for object files"
section_comdat = "Section contains COMDAT data. Valid only for object files"
section_no_defer_spec_exc = "Reset speculative exceptions handling bits in TLB entries for this section"
section_gprel = "Section contains data referenced through global pointer"
section_align_1bytes = "Align data on 1-byte boundary. Valid only for object files"
section_align_2bytes = "Align data on 2-byte boundary. Valid only for object files"
section_align_4bytes = "Align data on 4-byte boundary. Valid only for object files"
section_align_8bytes = "Align data on 8-byte boundary. Valid only for object files"
section_align_16bytes = "Align data on 16-byte boundary. Valid only for object files"
section_align_32bytes = "Align data on 32-byte boundary. Valid only for object files"
section_align_64bytes = "Align data on 64-byte boundary. Valid only for object files"
section_align_128bytes = "Align data on 128-byte boundary. Valid only for object files"
section_align_256bytes = "Align data on 256-byte boundary. Valid only for object files"
section_align_512bytes = "Align data on 512-byte boundary. Valid only for object files"
section_align_1024bytes = "Align data on 1024-byte boundary. Valid only for object files"
section_align_2048bytes = "Align data on 2048-byte boundary. Valid only for object files"
section_align_4096bytes = "Align data on 4096-byte boundary. Valid only for object files"
section_align_8192bytes = "Align data on 8192-byte boundary. Valid only for object files"
section_reloc_ovfl = "Section contains extended relocations. Relocation count exceeds 16 bits reserved for it in section header"
section_mem_discardable = "Section can be discarded as needed"
section_mem_not_cached = "Section cannot be cached"
section_mem_not_paged = "Section cannot be paged"
section_mem_shared = "Section can be shared in memory"
section_mem_execute = "Section can be executed as code"
section_mem_read = "Section can be read"
section_mem_write = "Section can be written"

# DLL debugging related
function_not_found = "Function not found"
"#.to_string()
}

fn create_default_config() -> toml::Value {
    toml::from_str(&create_default_config_string()).unwrap()
}

fn generate_constants(language: &str, lang_config: &toml::Value) -> String {
    let mut constants = String::new();
    constants.push_str("// 自动生成的常量\n");
    constants.push_str("#[allow(dead_code)]\n");
    constants.push_str("pub const CURRENT_LANGUAGE: &str = \"");
    constants.push_str(language);
    constants.push_str("\";\n\n");

    // 为每个配置项生成常量
    if let Some(table) = lang_config.as_table() {
        for (key, value) in table {
            if let Some(str_value) = value.as_str() {
                constants.push_str("pub const ");
                constants.push_str(&key.to_uppercase());
                constants.push_str(": &str = \"");
                constants.push_str(str_value);
                constants.push_str("\";\n");
            }
        }
    }

    constants
}
