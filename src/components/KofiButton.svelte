<!--
copied from the kofi script so we dont need to load another script
-->

<span title="Support us">
    <div class="btn-container">
        <a
            title="Support us!"
            class="kofi-button"
            style="background-color:#7ADE2D;"
            href="https://ko-fi.com/Z8Z410GRY2"
            target="_blank"
        >
            <span class="kofitext">
                <img
                    src="https://storage.ko-fi.com/cdn/cup-border.png"
                    alt="Ko-fi donations"
                    class="kofiimg"
                />
                Support Us!
            </span>
        </a>
    </div>
</span>

<style>
    .kofiimg {
        display: initial !important;
        vertical-align: middle;
        height: 13px !important;
        width: 20px !important;
        padding-top: 0 !important;
        padding-bottom: 0 !important;
        border: none;
        margin-top: 0;
        margin-right: 5px !important;
        margin-left: 0 !important;
        margin-bottom: 3px !important;
        content: url("https://storage.ko-fi.com/cdn/cup-border.png");
    }

    .kofiimg:after {
        vertical-align: middle;
        height: 25px;
        padding-top: 0;
        padding-bottom: 0;
        border: none;
        margin-top: 0;
        margin-right: 6px;
        margin-left: 0;
        margin-bottom: 4px !important;
        content: url("https://storage.ko-fi.com/cdn/whitelogo.svg");
    }
    .btn-container {
        display: inline-block !important;
        white-space: nowrap;
        min-width: 160px;
    }
    .kofitext {
        color: #13240c !important;
        letter-spacing: -0.15px !important;
        text-wrap: none;
        vertical-align: middle;
        line-height: 33px !important;
        padding: 0;
        text-align: center;
        text-decoration: none !important;
        text-shadow: 0 1px 1px rgba(34, 34, 34, 0.05);
    }
    .kofi-button {
        box-shadow: 1px 1px 0px rgba(0, 0, 0, 0.2);
        line-height: 36px !important;
        min-width: 150px;
        display: inline-block !important;
        background-color: #29abe0;
        padding: 2px 12px !important;
        text-align: center !important;
        border-radius: 7px;
        color: #fff;
        cursor: pointer;
        overflow-wrap: break-word;
        vertical-align: middle;
        border: 0 none #fff !important;
        text-decoration: none;
        text-shadow: none;
        font-weight: 700 !important;
        font-size: 14px !important;
    }
    .kofi-button:visited {
        color: #fff !important;
        text-decoration: none !important;
    }
    .kofi-button:hover {
        opacity: 0.85;
        color: #f5f5f5 !important;
        text-decoration: none !important;
    }
    .kofi-button:active {
        color: #f5f5f5 !important;
        text-decoration: none !important;
    }
    .kofitext .kofiimg {
        height: 15px !important;
        width: 22px !important;
        display: initial;
        animation: kofi-wiggle 3s infinite;
    }
    @keyframes kofi-wiggle {
        0% {
            transform: rotate(0) scale(1);
        }
        60% {
            transform: rotate(0) scale(1);
        }
        75% {
            transform: rotate(0) scale(1.12);
        }
        80% {
            transform: rotate(0) scale(1.1);
        }
        84% {
            transform: rotate(-10deg) scale(1.1);
        }
        88% {
            transform: rotate(10deg) scale(1.1);
        }
        92% {
            transform: rotate(-10deg) scale(1.1);
        }
        96% {
            transform: rotate(10deg) scale(1.1);
        }
        100% {
            transform: rotate(0) scale(1);
        }
    }
</style>
