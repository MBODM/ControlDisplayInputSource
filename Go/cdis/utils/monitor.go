package utils

import (
	"fmt"
	"syscall"
	"unsafe"
)

type CompositeMonitorInfo struct {
	PhysicalInfo PhysicalMonitorInfo
	SysInfo      SystemMonitorInfo
}

type SystemMonitorInfo struct {
	Handle        syscall.Handle
	DeviceContext syscall.Handle
	RectAngle     RECT
}

type PhysicalMonitorInfo struct {
	Handle      syscall.Handle
	Description string
}

type RECT struct {
	Left   int32
	Top    int32
	Right  int32
	Bottom int32
}

// GetCompositeMonitors
func GetCompositeMonitors() (monitors []CompositeMonitorInfo, err error) {
	systemMonitors, err := GetSystemMonitors()
	if err != nil {
		return nil, err
	}
	for _, sysMonitor := range systemMonitors {
		physicalMonitor, err := GetPhysicalMonitor(sysMonitor.Handle)
		if err != nil {
			continue
		}
		monitors = append(monitors, CompositeMonitorInfo{
			PhysicalInfo: physicalMonitor,
			SysInfo:      sysMonitor,
		})
	}
	return monitors, nil
}

// GetSystemMonitors
func GetSystemMonitors() (info []SystemMonitorInfo, err error) {
	// https://docs.microsoft.com/en-us/windows/win32/api/winuser/nc-winuser-monitorenumproc
	var fnCallback = func(hMonitor syscall.Handle, hdc syscall.Handle, rect *RECT, lParam uintptr) int {
		info = append(info, SystemMonitorInfo{Handle: hMonitor, DeviceContext: hdc, RectAngle: *rect})
		return 1
	}
	// uintptr(unsafe.Pointer(nil)) or uintptr(syscall.Handle(0))
	_, _, callErr := syscall.SyscallN(procEnumDisplayMonitors,
		uintptr(unsafe.Pointer(nil)),
		uintptr(unsafe.Pointer(nil)),
		syscall.NewCallback(fnCallback),
		uintptr(unsafe.Pointer(nil)),
	)
	if callErr != 0 {
		return info, fmt.Errorf(callErr.Error())
	}
	return info, nil
}

// GetMonitorNumberFromHandle
func GetMonitorNumberFromHandle(hMonitor syscall.Handle) (number int32, err error) {
	_, _, callErr := syscall.SyscallN(procGetNumberOfPhysicalMonitorsFromHMONITOR,
		uintptr(hMonitor),
		uintptr(number),
	)
	if callErr != 0 {
		return 0, fmt.Errorf(callErr.Error())
	}
	return number, nil
}

// GetPhysicalMonitor
func GetPhysicalMonitor(hMonitor syscall.Handle) (info PhysicalMonitorInfo, err error) {
	bytes := make([]byte, 256)
	_, _, callErr := syscall.SyscallN(procGetPhysicalMonitorsFromHMONITOR,
		uintptr(hMonitor),
		uintptr(1),
		uintptr(unsafe.Pointer(&bytes[0])),
	)
	if callErr != 0 {
		return PhysicalMonitorInfo{}, fmt.Errorf(callErr.Error())
	}
	var newBytes []byte
	for _, b := range bytes[8:] {
		if b != 0 {
			newBytes = append(newBytes, b)
		}
	}
	return PhysicalMonitorInfo{Handle: syscall.Handle(bytes[0]), Description: string(newBytes)}, nil
}
