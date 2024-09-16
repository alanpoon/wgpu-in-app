//
//  ViewController.swift
//
//  Created by LiJinlei on 2021/9/10.
//

import UIKit

class ViewController: UIViewController，WKNavigationDelegate {
    @IBOutlet var metalV: MetalView!
    var wgpuCanvas: OpaquePointer?
    var webView: WKWebView!
    var once = false
    lazy var displayLink: CADisplayLink = {
        CADisplayLink.init(target: self, selector: #selector(enterFrame))
    }()
    
    override func viewDidLoad() {
        super.viewDidLoad()
       print("viewDidLoad")
        self.displayLink.add(to: .current, forMode: .default)
        self.displayLink.isPaused = true
        let webViewConfiguration = WKWebViewConfiguration()
        webView = WKWebView(frame: .zero, configuration: webViewConfiguration)
        webView.navigationDelegate = self
        view = webView
    }
    
    override func viewDidAppear(_ animated: Bool) {
        print("viewDidAppear")
        if let resourcePath = Bundle.main.resourcePath {
            let resourceURL = URL(fileURLWithPath: resourcePath).appendingPathComponent("test.rtf")
            print("File URL: \(resourceURL)")
            do {
                print("content")
                let content = try String(contentsOf: resourceURL, encoding: .utf8)
                
                print(content)
            } catch{
                print("\(error)")
            }
        }
        super.viewDidAppear(animated)
        self.view.backgroundColor = .white
        if wgpuCanvas == nil {	
            let viewPointer = Unmanaged.passUnretained(self.metalV).toOpaque()
            let metalLayer = Unmanaged.passUnretained(self.metalV.layer).toOpaque()
            let maximumFrames = UIScreen.main.maximumFramesPerSecond
            
            let viewObj = ios_view_obj(view: viewPointer, metal_layer: metalLayer,maximum_frames: Int32(maximumFrames), callback_to_swift: callback_to_swift)
            
            wgpuCanvas = create_wgpu_canvas(viewObj)
        }
        self.displayLink.isPaused = false
    }
    
    override func viewWillDisappear(_ animated: Bool) {
        super.viewWillDisappear(animated)
        displayLink.isPaused = true
    }
    @IBAction func changeExample(sender: UISegmentedControl) {
        guard let canvas = self.wgpuCanvas else {
                return
        }
        var index = sender.selectedSegmentIndex
        // if index == 2 {
        //     index = 5
        // }
        if index !=0{
            webView.idHidden = true
            metalV.isHidden = false
        }else{
            let url = URL(string:"https://ambient.run/packages/h3gv2vnpcajq75woh5nmiemeahfpaku4")!
            webView.load(URLRequest(url: url))
            webView.allowsBackForwardNavigationGestures = true
            webView.configuration.defaultWebpagePreferences.allowsContentJavaScript = true
            metalV.isHidden = true
            webView.isHidden = false
        }
        //change_example(canvas, Int32(index))
    }
    @objc func enterFrame() {
        
        guard let canvas = self.wgpuCanvas else {
            return
        }
        
        // call rust
        if !once{
            print("enterFrame")
            enter_frame(canvas)
            once = true
            print("enterFrame after")
        }
        
    }
    

}

func callback_to_swift(arg: Int32) {
    DispatchQueue.main.async {
        switch arg {
        case 0:
            print("wgpu canvas created!")
            break
        case 1:
            print("canvas enter frame")
            break
            
        default:
            break
        }
    }
    
}
