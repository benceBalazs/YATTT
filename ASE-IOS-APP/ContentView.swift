//
//  ContentView.swift
//  ASE-IOS-APP
//
//  Created by Laura likar on 15.11.24.
//

import SwiftUI

struct ContentView: View {
    // Theme for dark mode
    @State private var isDarkMode = false
    
    
    var body: some View {
        TabView {
            
            // Home Tab
            HomeView()
                .tabItem {
                    Image(systemName: "house")
                    Text("Home")
                }
            
            // Settings Tab
            SettingsView(isDarkMode: $isDarkMode)
                .tabItem {
                    Image(systemName: "gear")
                    Text("Settings")
                }
        }
        //Theme
        .preferredColorScheme(isDarkMode ? .dark : .light)
        .animation(.easeInOut, value: isDarkMode)
    }
}

struct HomeView: View {
    @State private var rfcData: String = ""
    @State private var responseMessage: String = ""
    var body: some View {
        VStack {
            Text("Reader")
                .font(.largeTitle)
                .padding()
            
            Spacer()
            
            
            Button(action: {
                handleNFC()
            }) {
                Text("NFC")
                    .font(.headline)
                    .padding()
                    .frame(maxWidth: .infinity)
                    .background(Color.blue)
                    .foregroundColor(.white)
                    .cornerRadius(10)
            }
            .padding(.horizontal)
            Button(action: {
                handleRFID()
            }) {
                Text("RFID")
                    .font(.headline)
                    .padding()
                    .frame(maxWidth: .infinity)
                    .background(Color.green)
                    .foregroundColor(.white)
                    .cornerRadius(10)
            }
            .padding(.horizontal)
            
            Text(responseMessage)
                .padding()
                .foregroundColor(responseMessage.contains("Error") ? .red : .green)
            Spacer()
            
        }
        .padding()
    }
    
    //NFC
    private func handleNFC() {
        let nfcData = simulateData() // Simulated NFC data
        print("NFC Emulation Data: \(nfcData)")
        sendDataToApi(data: nfcData, emulationType: "NFC")
    }
    //RFC
    private func handleRFID() {
        let rfidData = simulateData()
        print("RFID Emulation Data: \(rfidData)")
        sendDataToApi(data: rfidData, emulationType: "RFID")
    }
    
    // Send Data to API
    private func sendDataToApi(data: String, emulationType: String) {
        // get the deviceId
        guard let deviceId = UIDevice.current.identifierForVendor?.uuidString else {
            responseMessage = "Error: Unable to fetch device ID"
            return
        }
        print("Device ID:", deviceId)

        // API call with RFID/NFC data and device ID
        Api.shared.sendData(rfcData: data, deviceId: deviceId) { result in
            DispatchQueue.main.async {
                switch result {
                case .success(let response):
                    responseMessage = "\(emulationType) Success: \(response)"
                case .failure(let error):
                    responseMessage = "\(emulationType) Error: \(error.localizedDescription)"
                }
            }
        }
        

    }

    
    private func simulateData() -> String {
        // Simulated random RFC data
        let randomRFC = UUID().uuidString.prefix(8)
        print("Generated number: ", randomRFC)
        return String(randomRFC)
    }
}

struct SettingsView: View {
    @Binding var isDarkMode: Bool
    
    var body: some View {
        VStack(alignment: .leading) {
            Text("Settings")
                .font(.largeTitle)
                .padding(.top, 20)
                .padding(.horizontal)
            
            Toggle(isOn: $isDarkMode) {
                Text("Dark Mode")
                    .font(.headline)
            }
            .padding(.horizontal)
            Spacer()
            
        }
    }
}


#Preview {
    ContentView()
}

