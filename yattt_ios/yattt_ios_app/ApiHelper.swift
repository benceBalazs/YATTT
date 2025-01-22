//
//  apiHelper.swift
//  ASE-IOS-APP
//
//  Created by Laura likar on 19.11.24.
//

import Foundation

struct ApiHelper {
    static func createRequest(url: String, method: String, body: [String: Any]?) -> URLRequest? {
        //guard for ensure the URL is valid
        guard let url = URL(string: url) else {
            print("Invalid URL: \(url)")
            return nil
        }

        var request = URLRequest(url: url)
        request.httpMethod = method
        request.addValue("application/json", forHTTPHeaderField: "Content-Type")
        
        if let body = body {
            if let jsonBody = try? JSONSerialization.data(withJSONObject: body) {
                request.httpBody = jsonBody
                
                // Log the request details
                print("Request Created:")
                print("URL: \(url)")
                print("Method: \(method)")
                print("Headers: \(request.allHTTPHeaderFields ?? [:])")
                print("Body: \(String(data: jsonBody, encoding: .utf8) ?? "Invalid JSON")")
            } else {
                print("Failed to serialize request body")
                return nil
            }
        }
        
        return request
    }
    
    static func parseResponse(data: Data?, response: URLResponse?, error: Error?) -> Result<Data, Error> {
        if let error = error {
            print("Network Error: \(error.localizedDescription)")
            return .failure(error)
        }
        
        if let httpResponse = response as? HTTPURLResponse {
            print("Response Status Code: \(httpResponse.statusCode)")
        }
        
        if let data = data {
            print("Response Data: \(String(data: data, encoding: .utf8) ?? "Invalid Data")")
            return .success(data)
        }
        
        return .failure(NSError(domain: "", code: -1, userInfo: [NSLocalizedDescriptionKey: "No Data"]))
    }
}

