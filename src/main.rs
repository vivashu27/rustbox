#![allow(unused_imports)]
use winapi::um::wininet::InternetCheckConnectionA;
use std::ffi::c_long;
use std::process::{exit,Command};
use std::os::raw::c_double;
use std::mem;
use std::fs;
use winapi::shared::minwindef::HKEY;
use winapi::um::synchapi::Sleep;
use std::ptr::null_mut;
use  winapi::um::tlhelp32::*;
use winapi::um::minwinbase::SYSTEMTIME;
use winapi::shared::ntdef::ULARGE_INTEGER;
use winapi::um::sysinfoapi::GetTickCount;
use winapi::um::sysinfoapi::{GetSystemInfo,SYSTEM_INFO};
use winapi::shared::minwindef::DWORD;
use winapi::um::sysinfoapi::{GetSystemTime,LPMEMORYSTATUSEX,MEMORYSTATUSEX,GlobalMemoryStatusEx};
use winapi::um::fileapi::GetDiskFreeSpaceExW;
use winapi::um::winreg::{RegOpenKeyExA, RegCloseKey};
use std::ffi::CStr;
//tweak the properties according to you desire
pub fn main(){
    if (check_users() && check_hostname() && check_RAM() && check_process() && check_uptime() && check_hardisk_size() && sleep_disallowed() && check_network_isolation() && check_files()&&check_reg()||check_process()){
        println!("Sandbox detected!!");

    }
    else{
        println!("Its all fine!!");
    }
}
//check if some of the common sandbox process is running or not
fn check_process()->bool{
    unsafe{
        let mut res=false;
        let process=["vboxservice.exe",
		"vboxtray.exe",
		//VirtualPC
		"vmsrvc.exe",
		"vmusrvc.exe",
        
		//VMWare
		"vmtoolsd.exe",
		"vmacthlp.exe",
		"vmwaretray.exe",
		"vmwareuser.exe",
		"vmware.exe",
		"vmount2.exe",
		//Xen
		"xenservice.exe",
		"xsvc_depriv.exe"];
         let pshandle:winapi::um::winnt::HANDLE=CreateToolhelp32Snapshot(0x00000002, 0);
         if pshandle != winapi::um::handleapi::INVALID_HANDLE_VALUE{
            let mut psentry:PROCESSENTRY32=std::mem::zeroed();
            psentry.dwSize = std::mem::size_of::<PROCESSENTRY32>() as u32;
            
            for i in &process{
                if Process32First(pshandle, &mut psentry) !=0{
                while(Process32Next(pshandle, &mut psentry)!=0){
                    let exe_file = CStr::from_ptr(psentry.szExeFile.as_ptr());
                    let exe_file_str = exe_file.to_str().unwrap();
                    
                    if(i==&exe_file_str){
                        res=true;
                    }
                }
            }
        }
         }
         res
    }
}
//check if common registries exits 
fn check_reg()->bool{
    unsafe{
        let mut res=false;
        let reg=["HARDWARE\\ACPI\\DSDT\\VBOX__",
		"HARDWARE\\ACPI\\FADT\\VBOX__",
		"HARDWARE\\ACPI\\RSDT\\VBOX__",
		"SOFTWARE\\Oracle\\VirtualBox Guest Additions",
		"SYSTEM\\ControlSet001\\Services\\VBoxGuest",
		"SYSTEM\\ControlSet001\\Services\\VBoxMouse",
		"SYSTEM\\ControlSet001\\Services\\VBoxService",
		"SYSTEM\\ControlSet001\\Services\\VBoxSF",
		"SYSTEM\\ControlSet001\\Services\\VBoxVideo",
		//VMware
		"SOFTWARE\\VMware, Inc.\\VMware Tools",
		"SYSTEM\\ControlSet001\\Services\\vmdebug",
		"SYSTEM\\ControlSet001\\Services\\vmmouse",
		"SYSTEM\\ControlSet001\\Services\\VMTools",
		"SYSTEM\\ControlSet001\\Services\\VMMEMCTL",
		"SYSTEM\\ControlSet001\\Services\\vmware",
		"SYSTEM\\ControlSet001\\Services\\vmci",
		"SYSTEM\\ControlSet001\\Services\\vmx86",
		//Wine
		"SOFTWARE\\Wine",
		//Xen
		"HARDWARE\\ACPI\\DSDT\\xen",
		"HARDWARE\\ACPI\\FADT\\xen",
		"HARDWARE\\ACPI\\RSDT\\xen",
		"SYSTEM\\ControlSet001\\Services\\xenevtchn",
		"SYSTEM\\ControlSet001\\Services\\xennet",
		"SYSTEM\\ControlSet001\\Services\\xennet6",
		"SYSTEM\\ControlSet001\\Services\\xensvc",
		"SYSTEM\\ControlSet001\\Services\\xenvdb"];
        for i in &reg{
            let mut hkey:HKEY=std::mem::zeroed();
            let result:c_long=RegOpenKeyExA(
                winapi::um::winreg::HKEY_LOCAL_MACHINE, 
                i.as_ptr() as *const i8, 
                0 as u32, 
                winapi::um::winnt::KEY_READ , 
                hkey as *mut *mut winapi::shared::minwindef::HKEY__
            );
            if result == winapi::shared::winerror::ERROR_SUCCESS as i32{
                if(result != winapi::shared::winerror::ERROR_FILE_NOT_FOUND as i32){
                        res=true;
                        println!("{}",i);
                }
            }
            RegCloseKey(hkey);
        }
        res
    }
}
//check if common directories exits for VM sandbox
fn check_directory()->bool{
    unsafe{
        let mut res=false;
        let dirs =["C:\\Program Files\\VMware\\VMware Tools",
		"C:\\Program Files\\Oracle\\VirtualBox Guest Additions"];
        for i in  &dirs{
            if fs::metadata(i).is_ok() {
                res=true;

            }
        }
        res
    }
}

//check the common VM files exits
fn check_files()->bool{
    unsafe{
        let mut res=false;
    let files=["C:\\windows\\system32\\drivers\\VBoxMouse.sys",
    "C:\\windows\\system32\\drivers\\VBoxGuest.sys",
    "C:\\windows\\system32\\drivers\\VBoxSF.sys",
    "C:\\windows\\system32\\drivers\\VBoxVideo.sys",
    "C:\\windows\\system32\\vboxdisp.dll",
    "C:\\windows\\system32\\vboxhook.dll",
    "C:\\windows\\system32\\vboxmrxnp.dll",
    "C:\\windows\\system32\\vboxogl.dll",
    "C:\\windows\\system32\\vboxoglarrayspu.dll",
    "C:\\windows\\system32\\vboxoglcrutil.dll",
    "C:\\windows\\system32\\vboxoglerrorspu.dll",
    "C:\\windows\\system32\\vboxoglfeedbackspu.dll",
    "C:\\windows\\system32\\vboxoglpackspu.dll",
    "C:\\windows\\system32\\vboxoglpassthroughspu.dll",
    "C:\\windows\\system32\\vboxservice.exe",
    "C:\\windows\\system32\\vboxtray.exe",
    "C:\\windows\\system32\\VBoxControl.exe",
    //VMware
    "C:\\windows\\system32\\drivers\\vmmouse.sys",
    "C:\\windows\\system32\\drivers\\vmnet.sys",
    "C:\\windows\\system32\\drivers\\vmxnet.sys",
    "C:\\windows\\system32\\drivers\\vmhgfs.sys",
    "C:\\windows\\system32\\drivers\\vmx86.sys",
    "C:\\windows\\system32\\drivers\\hgfs.sys"];
    for i in &files{
        if let Ok(metadata) = fs::metadata(i) {
            res=true;
        }
    }
    res
}
}
//If the system is isolated from the network
fn check_network_isolation()->bool{
    let mut res=false;
    unsafe{
    if(InternetCheckConnectionA("https://google.com".as_ptr() as *const i8,winapi::um::wininet::FLAG_ICC_FORCE_CONNECTION , 0)!=0){
        res=true;
    }
    res
}
}


//checking the sleep functionality is disallowed
fn sleep_disallowed()->bool{
    unsafe{ 
        let mut start:SYSTEMTIME=std::mem::zeroed();
        let mut end:SYSTEMTIME=std::mem::zeroed();
        let mut res=false;
        GetSystemTime(&mut start);
        Sleep(3000);
        GetSystemTime(&mut end);
        let startimems=(start.wSecond* 1000) + start.wMilliseconds;
        let endtimems = (end.wSecond* 1000) + end.wMilliseconds;
        if(endtimems-startimems<2800){
            res=true;

        }
        res
        }
   
}
//check the hardrive size
fn check_hardisk_size()->bool{
    unsafe{
    let mut res=false;
    let mut total_space: ULARGE_INTEGER=std::mem::zeroed();  
    let mut total_space_giga: u64;
    if(GetDiskFreeSpaceExW(null_mut(), 
    null_mut() as  *mut winapi::um::winnt::ULARGE_INTEGER, 
        &mut total_space, null_mut() as *mut winapi::um::winnt::ULARGE_INTEGER)!=0){
            total_space_giga = total_space.QuadPart() / (1024 * 1024* 1024);
            if(total_space_giga<= 100)
            {
                println!("{:?}",total_space_giga);
                res=true;
            }
            else{
                println!("HELE {}",total_space_giga);
            }
            
        }
        else{
            println!("Failed to fetch size!!!");
        }
        res
    }
}
//check uptime system is less than 5 minutes
fn check_uptime()->bool{
    unsafe{
    let mut tick:DWORD=std::mem::zeroed();
    tick= GetTickCount();
    let mut res=false;
    let min = tick/(1000*60);
    if(min<=5){
        res=true;
    }
    res
}
}
//checking the size of the RAM
fn check_RAM()->bool{
    unsafe{
        let mut res=false;
    let mut status:MEMORYSTATUSEX=std::mem::zeroed();
    status.dwLength = std::mem::size_of::<MEMORYSTATUSEX>() as DWORD;
    if(GlobalMemoryStatusEx(&mut status)!=0){
        let ram:c_double=status.ullTotalPhys as f64/(1024*1024*1024) as f64;
        if(ram>=4.0){
                res=false;
        }
        else{
            res=true;
        }
    }
    res
    }
}
//checking the current hostname
fn check_hostname() -> bool{
    let to_check= ["sandbox",
    "sandboxdetect",
    "john - pc",
    "mueller - pc",
    "virus",
    "malware",
    "hanspeter - pc",
    "malwaretest",
    "fortinet"];
    let mut cmd=Command::new("hostname");
    let mut res=false;
    let mut output = cmd.output().expect("Error executing code");
    let hostname=String::from_utf8(output.stdout);
    let mut current_host=String::new();
    match hostname{
        Ok(c)=>current_host=c,
        Err(err)=>println!("{}",err),
    }
    for i in &to_check{
        let mut lol =i as &str;
        if current_host.trim().to_lowercase() == lol.trim().to_lowercase(){
            
            res=true;
            break;
        }
    }
    res
}
//checking the current username
fn check_users() -> bool{
    let  to_check=["admin","andy","honey","EUser",
    "john",
    "john doe",
    "malnetvm",
    "maltest",
    "malware",
    "roo",
    "sandbox",
    "snort",
    "tequilaboomboom",
    "test",
    "virus",
    "virusclone",
    "wilbert",
    "nepenthes",
    "currentuser",
    "username",
    "user",
    "vmware"];
    let mut cmd=Command::new("whoami");
    let mut  res=false;
    let mut output = cmd.output().expect("Error executing code");
    let mut gg = String::from_utf8_lossy(&mut output.stdout);
    let only_user:Vec<&str>=gg.split("\\").collect();
    for i in &to_check{
        let mut lol =i as &str;
        if (only_user[1] as &str).trim().to_lowercase() == lol.trim().to_lowercase(){
            
            res=true;
            break;
        }
    }
    res
}