<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>Loading Bar</title>
    <style>
        body {
            background: #1d2630;
            font-family: sans-serif;
        }

        .container {
            width: 100%;
            min-height: 100vh;
            display: flex;
            justify-content: center;
            align-items: center;
            flex-direction: column;
        }

        .text {
            color: white;
            margin-bottom: 25px;
            font-size: 20px;
            letter-spacing: 2px;
        }

        .loading {
            width: 100%;
            height: auto;
            display: flex;
            justify-content: center;
            align-items: center;
        }

        .loading .line-box {
            padding: 2px;
            width: 40%;
            height: 20px;
            border: 2px solid #267591;
            border-radius: 20px;
            overflow: hidden; /* Mengatasi overflow agar animasi tidak terlihat di luar */
        }

        .loading .line-box .line {
            height: 20px;
            width: 0%;
            border-radius: 20px;
            background: #67d295;
            animation: loading 2s forwards cubic-bezier(0,0,0,0);
        }

        @keyframes loading {
            0% {
                width: 0%;
            }
            100% {
                width: 100%;
            }
        }
    </style>
</head>
<body>
    <div class="container">
        <div class="text">Memuat...</div>
        <div class="loading">
            <div class="line-box">
                <div class="line"></div>
            </div>
        </div>
    </div>
</body>
</html>
