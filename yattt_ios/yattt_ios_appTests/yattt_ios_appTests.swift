//
//  ASE_IOS_APPTests.swift
//  ASE-IOS-APPTests
//
//  Created by Laura likar on 15.11.24.
//


import XCTest
import Testing
@testable import yattt_ios_app

final class yattt_ios_appTests:XCTestCase {

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
        
        // Unwrap body and XCTUnwrap if it's nil
        let httpBody = try? XCTUnwrap(request?.httpBody, "The request body should not be null!!!")

        // Decode
        let decod = try? JSONSerialization.jsonObject(with: httpBody!, options: []) as? [String: String]
        XCTAssertEqual(decod, expectedBody)
    }

}
