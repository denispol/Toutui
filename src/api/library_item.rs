use reqwest::Client; 
use serde::{Deserialize, Serialize}; 
use std::error::Error; 
use color_eyre::eyre::{Result, Report}; // Use Report rather than Box<dyn Error> 
use std::process::Command;
use std::process::Output;

                                        

/// Play a Library Item or Podcast Episode
/// https://api.audiobookshelf.org/#play-a-library-item-or-podcast-episode

pub async fn play() -> Result<(), Box<dyn Error>> {
    let output: Output = Command::new("vlc")
        .arg("https://audiobook.nuagemagique.duckdns.org/hls/6bc1d51a-dc06-438e-8229-eb7b9311fe06/output.m3u8?token=eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJ1c2VySWQiOiI3NTk2MmQzMy05YmNmLTQyYzYtODY2ZC0yODcwYjQzYWM2MWYiLCJ1c2VybmFtZSI6ImFsYmFuIiwiaWF0IjoxNzMyNDUyMTEwfQ.VCiv72-0PxLhRdJen3KKi8BE_QDPBGmKQCNOzHf25lQ")
        .output()
        .expect("Failed to execute program");
Ok(())
}

