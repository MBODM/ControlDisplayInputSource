package utils

import (
	"fmt"
	"syscall"
	"unsafe"
)

// GetMonitorCapabilities
func GetMonitorCapabilities(hPhysicalMonitor syscall.Handle) (currentValue int, minimumValue int, maximumValue int, err error) {
	_, _, callErr := syscall.SyscallN(procGetMonitorCapabilities, uintptr(hPhysicalMonitor), uintptr(unsafe.Pointer(&minimumValue)), uintptr(unsafe.Pointer(&currentValue)), uintptr(unsafe.Pointer(&maximumValue)))
	if callErr != 0 {
		return currentValue, minimumValue, maximumValue, fmt.Errorf(callErr.Error())
	}
	return currentValue, minimumValue, maximumValue, nil
}

// GetMonitorBrightness
func GetMonitorBrightness(hPhysicalMonitor syscall.Handle) (currentValue int, minimumValue int, maximumValue int, err error) {
	_, _, callErr := syscall.SyscallN(procGetMonitorBrightness, uintptr(hPhysicalMonitor), uintptr(unsafe.Pointer(&minimumValue)), uintptr(unsafe.Pointer(&currentValue)), uintptr(unsafe.Pointer(&maximumValue)))
	if callErr != 0 {
		return currentValue, minimumValue, maximumValue, fmt.Errorf(callErr.Error())
	}
	return currentValue, minimumValue, maximumValue, nil
}

// SetMonitorBrightness
func SetMonitorBrightness(hPhysicalMonitor syscall.Handle, value int) (err error) {
	_, _, callErr := syscall.SyscallN(procSetMonitorBrightness, uintptr(hPhysicalMonitor), uintptr(value))
	if callErr != 0 {
		return fmt.Errorf(callErr.Error())
	}
	return nil
}

// GetMonitorContrast
func GetMonitorContrast(hPhysicalMonitor syscall.Handle) (currentValue int, minimumValue int, maximumValue int, err error) {
	_, _, callErr := syscall.SyscallN(procGetMonitorContrast, uintptr(hPhysicalMonitor), uintptr(unsafe.Pointer(&minimumValue)), uintptr(unsafe.Pointer(&currentValue)), uintptr(unsafe.Pointer(&maximumValue)))
	if callErr != 0 {
		return currentValue, minimumValue, maximumValue, fmt.Errorf(callErr.Error())
	}
	return currentValue, minimumValue, maximumValue, nil
}

// SetMonitorContrast
func SetMonitorContrast(hPhysicalMonitor syscall.Handle, value int) (err error) {
	_, _, callErr := syscall.SyscallN(procSetMonitorContrast, uintptr(hPhysicalMonitor), uintptr(value))
	if callErr != 0 {
		return fmt.Errorf(callErr.Error())
	}
	return nil
}

// RestoreMonitorFactoryColorDefaults
func RestoreMonitorFactoryColorDefaults(hPhysicalMonitor syscall.Handle) (err error) {
	_, _, callErr := syscall.SyscallN(procRestoreMonitorFactoryColorDefaults, uintptr(hPhysicalMonitor))
	if callErr != 0 {
		return fmt.Errorf(callErr.Error())
	}
	return nil
}

// RestoreRestoreMonitorFactoryDefaults
func RestoreRestoreMonitorFactoryDefaults(hPhysicalMonitor syscall.Handle) (err error) {
	_, _, callErr := syscall.SyscallN(procRestoreMonitorFactoryDefaults, uintptr(hPhysicalMonitor))
	if callErr != 0 {
		return fmt.Errorf(callErr.Error())
	}
	return nil
}
