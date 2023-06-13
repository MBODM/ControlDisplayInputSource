package display2

import (
	"fmt"
	"syscall"
	"unsafe"
)

type (
	BOOL     bool
	DWORD    uint32
	HANDLE   uintptr
	HMONITOR HANDLE
	HWND     HANDLE
)

type RECT struct {
	Left, Top, Right, Bottom int32
}

type MONITORINFO struct {
	CbSize    uint32
	RcMonitor RECT
	RcWork    RECT
	DwFlags   uint32
}

type PHYSICAL_MONITOR struct {
	Monitor     HANDLE
	Description [128]uint16
}

var (
	user32 = syscall.NewLazyDLL("user32.dll")
	dxva2  = syscall.NewLazyDLL("dxva2.dll")

	getDesktopWindow                        = user32.NewProc("GetDesktopWindow")
	monitorFromWindow                       = user32.NewProc("MonitorFromWindow")
	getMonitorInfo                          = user32.NewProc("GetMonitorInfoW")
	getNumberOfPhysicalMonitorsFromHMONITOR = dxva2.NewProc("GetNumberOfPhysicalMonitorsFromHMONITOR")
	getPhysicalMonitorsFromHMONITOR         = dxva2.NewProc("GetPhysicalMonitorsFromHMONITOR")
)

func GetDesktopWindow() HWND {
	ret, _, _ := getDesktopWindow.Call()
	return HWND(ret)
}

const (
	MONITOR_DEFAULTTONULL    = 0x0
	MONITOR_DEFAULTTOPRIMARY = 0x1
	MONITOR_DEFAULTTONEAREST = 0x2
)

func MonitorFromWindow(hwnd HWND, dwFlags uint32) (HMONITOR, error) {
	ret, _, err := monitorFromWindow.Call(
		uintptr(hwnd),
		uintptr(dwFlags),
	)
	if err != nil {

		// Was mach ich hier ?
		if syserr, ok := err.(syscall.Errno); ok {

			fmt.Println("RABAZZZ:", uint64(syserr))
			if syserr == 1060 {
				//Do whatever
			}
		}

		//fmt.Println("RABAZZZ:", int64(syscall.Errno))
		return HMONITOR(0), fmt.Errorf("Error in MonitorFromWindow() function: %w", err)
	}
	return HMONITOR(ret), nil
}

func GetMonitorInfo(hMonitor HMONITOR, lmpi *MONITORINFO) bool {
	if lmpi != nil {
		lmpi.CbSize = uint32(unsafe.Sizeof(*lmpi))
	}
	ret, _, _ := getMonitorInfo.Call(
		uintptr(hMonitor),
		uintptr(unsafe.Pointer(lmpi)),
	)
	return ret != 0
}

func GetNumberOfPhysicalMonitorsFromHMONITOR(monitor HMONITOR) (bool, DWORD) {
	var n DWORD
	ret, _, _ := getNumberOfPhysicalMonitorsFromHMONITOR.Call(
		uintptr(monitor),
		uintptr(unsafe.Pointer(&n)),
	)
	return ret != 0, n
}

// https://learn.microsoft.com/de-de/windows/win32/api/physicalmonitorenumerationapi/nf-physicalmonitorenumerationapi-getphysicalmonitorsfromhmonitor
// BOOL GetPhysicalMonitorsFromHMONITOR(
// [in]  HMONITOR           hMonitor,
// [in]  DWORD              dwPhysicalMonitorArraySize,
// [out] LPPHYSICAL_MONITOR pPhysicalMonitorArray);
func GetPhysicalMonitorsFromHMONITOR(hMonitor HMONITOR, dwPhysicalMonitorArraySize DWORD, physicalMonitorArray []PHYSICAL_MONITOR) BOOL {
	ret, _, _ := getPhysicalMonitorsFromHMONITOR.Call(
		uintptr(hMonitor),
		uintptr(dwPhysicalMonitorArraySize),
		uintptr(unsafe.Pointer(&physicalMonitorArray[0])),
	)
	return ret != 0
}
