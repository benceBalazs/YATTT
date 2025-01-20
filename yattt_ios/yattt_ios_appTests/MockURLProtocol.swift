//
//  MockURLProtocol.swift
//  yattt_ios_app
//
//  Created by Laura likar on 20.01.25.
//
import Foundation


//Mocking
class MockURLProtocol: URLProtocol {
    static var responseData: Data?
    static var responseError: Error?
    
    override class func canInit(with request: URLRequest) -> Bool {
        return true
    }
    
    override class func canonicalRequest(for request: URLRequest) -> URLRequest {
        return request
    }
    
    override func startLoading() {
        if let responseData = MockURLProtocol.responseData {
            self.client?.urlProtocol(self, didLoad: responseData)
        }
        if let responseError = MockURLProtocol.responseError {
            self.client?.urlProtocol(self, didFailWithError: responseError)
        }
        self.client?.urlProtocolDidFinishLoading(self)
    }
    
    override func stopLoading() {}
}
