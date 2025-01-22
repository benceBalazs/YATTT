//
//  ASE_IOS_APPTests.swift
//  ASE-IOS-APPTests
//
//  Created by Laura likar on 15.11.24.
//


import XCTest
import Testing
@testable import yattt_ios_app

//Mocking
/*class MockURLProtocol: URLProtocol {
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
}*/



final class yattt_ios_appTests:XCTestCase {
    
    let data = "testData"
    let deviceId = "testDeviceID"
    let url = "http://test1.com"
    let method = "POST"
    
    /*@Test func example() async throws {
     // Write your test here and use APIs like `#expect(...)` to check expected conditions.
     }*/
    
    var sut: Api! // System Under Test
    var session: URLSession!
    
    override func setUp() {
        super.setUp()
        let config = URLSessionConfiguration.ephemeral
        config.protocolClasses = [MockURLProtocol.self]
        session = URLSession(configuration: config)
        sut = Api(session: session)
    }
    
    override func tearDown() {
        sut = nil
        super.tearDown()
    }
    
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
    //Test API
    func testSendData_Success() {
        
        let expectedResponse = "Success response"
        MockURLProtocol.responseData = expectedResponse.data(using: .utf8)
        MockURLProtocol.responseError = nil
        
        let expectation = self.expectation(description: "API Success Call")
        
        sut.sendData(tagData: "testData", deviceId: "testDeviceId") { result in
        
            switch result {
            case .success(let response):
                XCTAssertEqual(response, expectedResponse)
            case .failure:
                XCTFail("Expected success but received failure")
            }
            expectation.fulfill()
        }
        
        waitForExpectations(timeout: 1, handler: nil)
    }
    
    func testSendData_Failure() {
        
        let error = NSError(domain: "TestError", code: 500, userInfo: nil)
        MockURLProtocol.responseError = error
        MockURLProtocol.responseData = nil
        
        let expectation = self.expectation(description: "API Failure Call")
        
        sut.sendData(tagData: "testData", deviceId: "testDeviceId") { result in

            switch result {
            case .success:
                XCTFail("Expected failure but received success")
            case .failure(let responseError):
                XCTAssertEqual((responseError as NSError).code, 500)
            }
            expectation.fulfill()
        }
        
        waitForExpectations(timeout: 1, handler: nil)
    }
    
}
