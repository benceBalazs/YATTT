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
            
            // Home
            HomeView()
                .tabItem {
                    Image(systemName: "rectangle.and.text.magnifyingglass")
                    Text("Reader")
                }
            
            // Settings
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
    @State private var responseMessage: String = ""
    @State private var defaultTagId: String = "E16EE47D"

    @State private var defaultDeviceId: String = "065CA9D0-7703-48A2-8FC1"
    @State private var isDefaultTagId: Bool = true
    @State private var isDefaultDeviceId: Bool = true

    var body: some View {
        VStack {
            Text("Reader")
                .font(.largeTitle)
                .padding()
            
            Spacer()
            
            // tag_id
            VStack(alignment: .leading) {
                Text("Tag ID")
                    .font(.headline)
                TextField("", text: $defaultTagId, onEditingChanged: { isEditing in
                    if isEditing {
                        isDefaultTagId = false
                    }
                })
                .foregroundColor(isDefaultTagId ? .gray : .primary)
                .textFieldStyle(RoundedBorderTextFieldStyle())
            }
            
            .padding()
            
            // device_id, editing
            VStack(alignment: .leading) {
                Text("Device ID")
                    .font(.headline)
                TextField("", text: $defaultDeviceId, onEditingChanged: { isEditing in
                    if isEditing {
                        isDefaultDeviceId = false
                    }
                })
                .foregroundColor(isDefaultDeviceId ? .gray : .primary)
                .textFieldStyle(RoundedBorderTextFieldStyle())
            }
            .padding()
            
            Button(action: {
                sendDataToApi(tag_id: defaultTagId, device_id: defaultDeviceId)
            }) {
                Text("SCAN")
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
    
    // Send Data to API
    private func sendDataToApi(tag_id: String, device_id: String) {
        
        guard !defaultTagId.isEmpty, !defaultDeviceId.isEmpty else {
            responseMessage = "Error: Please enter Tag ID and Device ID!"
            return
        }
        print("Tag ID: \(defaultTagId), Device ID: \(defaultDeviceId)")
        print("Device ID:", defaultDeviceId)
        
        // API call with RFID data and device ID
        Api.shared.sendData(tagData: tag_id, deviceId: defaultDeviceId) { result in
            DispatchQueue.main.async {
                switch result {
                case .success(let response):
                    responseMessage = "Success: \(response)"
                case .failure(let error):
                    responseMessage = "Error: \(error.localizedDescription)"
                }
            }
        }
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

