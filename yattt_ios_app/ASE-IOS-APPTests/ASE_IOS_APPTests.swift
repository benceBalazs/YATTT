//
//  ASE_IOS_APPTests.swift
//  ASE-IOS-APPTests
//
//  Created by Laura likar on 15.11.24.
//

import Testing
import XCTest
@testable import ASE_IOS_APP

final class ASE_IOS_APPTests:XCTestCase {
    let data = "testData"
    let deviceId = "testDeviceID"
    let url = "http://test1.com"
    let method = "POST"
    
    /*@Test func example() async throws {
        // Write your test here and use APIs like `#expect(...)` to check expected conditions.
    }*/
    
    func testCreateRequest() {
        let body = ["data": data, "deviceId": deviceId]
        
        let request = ApiHelper.createRequest(url: url, method: method, body: body)
        
        XCTAssertNotNil(request)
    }
    
    func testCheckRequestMethod() {
        let body = ["data": data, "deviceId": deviceId]
        
        let request = ApiHelper.createRequest(url: url, method: method, body: body)
        
        XCTAssertEqual(request?.httpMethod, "POST")
    }
    
    func testCreateURL() {
        
        let body = ["data": data, "deviceId": deviceId]
        
        let request = ApiHelper.createRequest(url: url, method: method, body: body)
        
        // Validate the URL
        XCTAssertEqual(request?.url?.absoluteString, "http://test1.com")
    }

    func testRequestBody() {
        let body = ["data": data, "deviceId": deviceId]
        let expectedBody = ["data": data, "deviceId": deviceId]
        
        let request = ApiHelper.createRequest(url: url, method: method, body: body)
        
        
        if let httpBody = request?.httpBody {
            // Decode the HTTP body
            let decodedBody = try? JSONSerialization.jsonObject(with: httpBody, options: []) as? [String: String]
            
            XCTAssertEqual(decodedBody, expectedBody)
            
        } else {
            XCTFail("The request body should not be null!")
        }
    }
}
