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
    
    func sendData(rfcData: String, deviceId: String, completion: @escaping (Result<String, Error>) -> Void) {
        // Use ApiHelper to create a request, guard to check that the request is valid
        guard let request = ApiHelper.createRequest(
            url: "http://192.168.1.140/",
            method: "POST",
            body: ["rfcData": rfcData, "deviceId": deviceId]
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
