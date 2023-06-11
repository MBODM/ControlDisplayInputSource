package main

import (
	"fmt"
	"os"

	"github.com/mbodm/cdis/utils"
)

func main() {
	fmt.Println("Hello")
	utils.RunTest()
	monitors, err := utils.GetSystemMonitors()
	if err != nil {
		showErrorAndExit(err)
	}
	for i, monitor := range monitors {
		sss := fmt.Sprintf("Index: %v Handle: %v DeviceContext: %v", i, monitor.Handle, monitor.DeviceContext)
		fmt.Println(sss)
	}
	monitor := monitors[0]
	if len(monitors) == 1 {
		fmt.Println("There is just 1 monitor")
		fmt.Println("The montior´s handle is:", monitor.Handle)
	}
	num, err := utils.MyGetNumberOfPhysicalMonitorsFromHMONITOR(monitor.Handle)
	if err != nil {
		showErrorAndExit(err)
	}
	fmt.Println(num)
	physicalMonitor, err := utils.GetPhysicalMonitor(monitor.Handle)
	if err != nil {
		showErrorAndExit(err)
	}
	desc := physicalMonitor.Description
	fmt.Println("Physical monitor description:", desc)
	currentValue, _, err := utils.GetVCPFeatureAndVCPFeatureReply(physicalMonitor.Handle, 0x60)
	if err != nil {
		showErrorAndExit(err)
	}
	s := fmt.Sprintf("Current VCP 60 value: %02X", currentValue)
	fmt.Println(s)
	fmt.Println("--------------------------------------------------------------------------------")
	//capslen, err := utils.GetCapabilitiesStringLength(physicalMonitor.Handle)
	// if err != nil {
	// 	fmt.Println("hier passiert es")
	// 	fmt.Println(err)
	// 	showErrorAndExit(err)
	// }
	// fmt.Println("Len is:", capslen)
	caps, err := utils.CapabilitiesRequestAndCapabilitiesReply(physicalMonitor.Handle)
	if err != nil {
		fmt.Println("dieser")
		showErrorAndExit(err)
	}
	fmt.Println(caps)
}

func showErrorAndExit(err error) {
	println("ERROR:")
	println(err)
	os.Exit(1)
}
