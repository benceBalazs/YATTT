//
//  api.swift
//  ASE-IOS-APP
//
//  Created by Laura likar on 19.11.24.
//
import Foundation

struct Api {
    //only one instance should exist
    static let shared = Api()
    
    private let session: URLSession
    
    init(session: URLSession = .shared) {
        self.session = session
    }
    
    func sendData(tagData: String, deviceId: String, completion: @escaping (Result<String, Error>) -> Void) {
        // ApiHelper to create a request, guard to check that the request is valid
        guard let request = ApiHelper.createRequest(
            url: "http://localhost:8000/api/v1/auth-tokens/scan",
            method: "POST",
            body: ["tag_id": tagData, "device_id": deviceId]
        ) else {
            completion(.failure(NSError(domain: "", code: -1, userInfo: [NSLocalizedDescriptionKey: "Invalid URL"])))
            return
        }
        
        // Perform the request
        let task = session.dataTask(with: request) { data, response, error in
            switch ApiHelper.parseResponse(data: data, response: response, error: error) {
            case .success(let responseData):
                if let responseString = String(data: responseData, encoding: .utf8) {
                    completion(.success(responseString))
                } else {
                    completion(.failure(NSError(domain: "", code: -1, userInfo: [NSLocalizedDescriptionKey: "Unable to parse response"])))
                }
            case .failure(let error):
                completion(.failure(error))
            }
        }
        task.resume()
    }
}
