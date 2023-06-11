package utils

import (
	"fmt"
	"syscall"
	"unsafe"
)

func MyGetNumberOfPhysicalMonitorsFromHMONITOR(hMonitor syscall.Handle) (number int32, err error) {
	_, _, callErr := syscall.SyscallN(procGetNumberOfPhysicalMonitorsFromHMONITOR,
		uintptr(hMonitor),
		uintptr(unsafe.Pointer(&number)),
	)
	if callErr != 0 {
		return 0, fmt.Errorf(callErr.Error())
	}
	return number, nil
}

func GetCapabilitiesStringLength(hMonitor syscall.Handle) (dwCapabilitiesStringLengthInCharacters uint32, err error) {
	var fuzz uint32 = 0
	_, _, callErr := syscall.SyscallN(procGetCapabilitiesStringLength,
		uintptr(hMonitor),
		uintptr(unsafe.Pointer(&fuzz)),
	)
	if callErr != 0 {
		return 0, fmt.Errorf(callErr.Error())
	}
	return fuzz, nil
}

func CapabilitiesRequestAndCapabilitiesReply(hMonitor syscall.Handle) (caps string, err error) {
	dwLength, err := GetCapabilitiesStringLength(hMonitor)
	if err != nil {
		fmt.Println("hier passierts 1")
		return "", fmt.Errorf(err.Error())
	} else {
		fmt.Println("Cap String Len:", dwLength)
	}
	bytes := make([]byte, dwLength)
	bytesLen := uint32(len(bytes))
	fmt.Println("BYTES LEN: ", bytesLen)

	_, _, callErr := syscall.SyscallN(procCapabilitiesRequestAndCapabilitiesReply,
		uintptr(hMonitor),
		uintptr(unsafe.Pointer(&bytes[0])),
		uintptr(bytesLen),
	)
	if callErr != 0 {
		fmt.Println("hier passierts 2")
		return "", fmt.Errorf(callErr.Error())
	}
	var newBytes []byte
	for _, b := range bytes[8:] {
		if b != 0 {
			newBytes = append(newBytes, b)
		}
	}
	return string(newBytes), nil
}
