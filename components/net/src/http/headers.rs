// Copyright (c) 2017 RioCorp Inc.

//Global header for Shields (we assume this as an aggregate of the 4 shields)
header! { (XAuthShield, "X-AUTH-SHIELD") => [String] }
header! { (XAuthShieldFingerPrint, "X-AUTH-FINGER-PRINT-SHIELD") => [String] }
header! { (XAuthShieldFace, "X-AUTH-FACE-SHIELD") => [String] }
header! { (XAuthShieldIris, "X-AUTH-IRIS-SHIELD") => [String] }
header! { (XAuthShieldVoice, "X-AUTH-VOICE-SHIELD") => [String] }

header! { (XAuthRioOSEmail, "X-AUTH-RIOOS-EMAIL") => [String] }
//This api key is a persistent one, and isn't recommended for users.
//We will show a warning to users if they wish to use it.
header! { (XAuthRioOSApiKey, "X-AUTH-RIOOS-APIKEY") => [String] }

header! { (CacheControl, "Cache-Control") => [String] }
header! { (ContentDisposition, "Content-Disposition") => [String] }
header! { (XFileName, "X-Filename") => [String] }
header! { (ETag, "ETag") => [String] }
