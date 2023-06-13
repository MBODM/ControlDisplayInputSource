package main

import (
	"fmt"
	"os"
	"syscall"

	"github.com/mbodm/cdis/display"
	"github.com/mbodm/cdis/display2"
)

func main() {
	fmt.Println("Hello")
	display.RunTest()

	fmt.Println("Get desktop window handle...")
	var hWindow display2.HWND = display2.GetDesktopWindow()
	fmt.Println("Desktop window handle:", hWindow)

	fmt.Println("Get monitor handle...")
	var hMonitor, err = display2.MonitorFromWindow(17 /*hWindow*/, display2.MONITOR_DEFAULTTOPRIMARY)
	if err != nil {
		fmt.Println(err.Error())
		os.Exit(55)
	}
	fmt.Println("Monitor handle:", hMonitor)

	fmt.Println("Get number of physical monitors...")
	bResult, dwNumberOfPhysMons := display2.GetNumberOfPhysicalMonitorsFromHMONITOR(hMonitor)
	fmt.Println("Result (BOOL):", bResult)
	fmt.Println("Result (DWORD):", dwNumberOfPhysMons)

	fmt.Println("Get physical monitors...")
	physMons := make([]display2.PHYSICAL_MONITOR, dwNumberOfPhysMons)
	fmt.Println("Lenght of PhysMons:", len(physMons))
	display2.GetPhysicalMonitorsFromHMONITOR(hMonitor, dwNumberOfPhysMons, physMons)
	fmt.Println("Result (BOOL):", bResult)

	fmt.Println("Physical monitors:")

	// fff := physMons[0]
	// fff.

	for _, physMon := range physMons {

		fmt.Println("Monitor", physMon.Monitor)
		fmt.Println("Description", syscall.UTF16ToString(physMon.Description[:]))
	}

	os.Exit(1)

	// Ab hier ist ALT

	monitors, err := display.GetSystemMonitors()
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
	num, err := display.MyGetNumberOfPhysicalMonitorsFromHMONITOR(monitor.Handle)
	if err != nil {
		showErrorAndExit(err)
	}
	fmt.Println(num)
	physicalMonitor, err := display.GetPhysicalMonitor(monitor.Handle)
	if err != nil {
		showErrorAndExit(err)
	}
	desc := physicalMonitor.Description
	fmt.Println("Physical monitor description:", desc)
	currentValue, _, err := display.GetVCPFeatureAndVCPFeatureReply(physicalMonitor.Handle, 0x60)
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
	caps, err := display.CapabilitiesRequestAndCapabilitiesReply(physicalMonitor.Handle)
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
