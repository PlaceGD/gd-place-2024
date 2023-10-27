<script lang="ts">
    let gluh = false
</script>

<button
    on:click={() => {
        gluh = !gluh
    }}>bib</button
>

<svg
    width="280"
    height="280"
    viewBox="0 0 14 14"
    fill="none"
    xmlns="http://www.w3.org/2000/svg"
>
    <g clip-path="url(#clip0_307_266)">
        <rect width="14" height="14" rx="2" fill="black" />
        <g opacity="0.7" filter="url(#filter0_f_307_266)">
            <circle cx="8.5" cy="13.5" r="3.5" fill="#FF00E6" />
        </g>
        <g opacity="0.7" filter="url(#filter1_f_307_266)">
            <circle cx="-0.5" cy="3.5" r="3.5" fill="#FFD600" />
        </g>
        <g opacity="0.7" filter="url(#filter2_f_307_266)">
            <circle cx="13.5" cy="0.5" r="3.5" fill="#00FFB3" />
        </g>
        <path d="M7 6L5 10H9L7 6Z" fill="url(#paint0_linear_307_266)" />
        <path
            d="M8.75729 9.85H5.24271L7 6.33541L8.75729 9.85Z"
            stroke="white"
            stroke-opacity="0.75"
            stroke-width="0.3"
        />
        <g filter="url(#filter3_d_307_266)">
            <path
                d="M7.13416 1.93292L7 1.66459L6.86584 1.93292L1.86584 11.9329L1.75729 12.15H2H4H4.09271L4.13416 12.0671L7 6.33541L8.7573 9.85H7H6.90729L6.86584 9.93292L5.86584 11.9329L5.75729 12.15H6H12H12.2427L12.1342 11.9329L7.13416 1.93292Z"
                stroke="white"
                stroke-width="0.3"
                shape-rendering="crispEdges"
            />
        </g>
        <path d="M3 10L2 12H4L3 10Z" fill="white" fill-opacity="0.2" />
        <path d="M4 8L3 10H5L4 8Z" fill="#FFB080" />
        <path d="M4 12L5 10L3 10L4 12Z" fill="#FF8080" />
        <path d="M5 10L6 8L4 8L5 10Z" fill="#FFDF80" />
        <path d="M6 8L7 6L5 6L6 8Z" fill="#C0FF80" />
        <path d="M7 6L8 4L6 4L7 6Z" fill="#80FFA0" />
        <path d="M8 8L9 6L7 6L8 8Z" fill="#80FFFF" />
        <path d="M9 10L10 8L8 8L9 10Z" fill="#80A0FF" />
        <path d="M10 12L11 10L9 10L10 12Z" fill="#C080FF" />
        <path d="M8 12L9 10L7 10L8 12Z" fill="#FF80DF" />
        <path d="M5 6L4 8H6L5 6Z" fill="#EFFF80" />
        <path d="M6 4L5 6H7L6 4Z" fill="#90FF80" />
        <path d="M9 10L8 12H10L9 10Z" fill="#EF80FF" />
        <path d="M7 2L6 4H8L7 2Z" fill="white" fill-opacity="0.4" />
        <path d="M8 4L7 6H9L8 4Z" fill="#80FFCF" />
        <path d="M9 6L8 8H10L9 6Z" fill="#80CFFF" />
        <path d="M10 8L9 10H11L10 8Z" fill="#9080FF" />
        {#if gluh}
            <path d="M11 10L10 12H12L11 10Z" fill="white" />
        {/if}
        <path d="M7 10L6 12H8L7 10Z" fill="#FF80B0" />
    </g>
    <defs>
        <filter
            id="filter0_f_307_266"
            x="-10"
            y="-5"
            width="37"
            height="37"
            filterUnits="userSpaceOnUse"
            color-interpolation-filters="sRGB"
        >
            <feFlood flood-opacity="0" result="BackgroundImageFix" />
            <feBlend
                mode="normal"
                in="SourceGraphic"
                in2="BackgroundImageFix"
                result="shape"
            />
            <feGaussianBlur
                stdDeviation="7.5"
                result="effect1_foregroundBlur_307_266"
            />
        </filter>
        <filter
            id="filter1_f_307_266"
            x="-19"
            y="-15"
            width="37"
            height="37"
            filterUnits="userSpaceOnUse"
            color-interpolation-filters="sRGB"
        >
            <feFlood flood-opacity="0" result="BackgroundImageFix" />
            <feBlend
                mode="normal"
                in="SourceGraphic"
                in2="BackgroundImageFix"
                result="shape"
            />
            <feGaussianBlur
                stdDeviation="7.5"
                result="effect1_foregroundBlur_307_266"
            />
        </filter>
        <filter
            id="filter2_f_307_266"
            x="-5"
            y="-18"
            width="37"
            height="37"
            filterUnits="userSpaceOnUse"
            color-interpolation-filters="sRGB"
        >
            <feFlood flood-opacity="0" result="BackgroundImageFix" />
            <feBlend
                mode="normal"
                in="SourceGraphic"
                in2="BackgroundImageFix"
                result="shape"
            />
            <feGaussianBlur
                stdDeviation="7.5"
                result="effect1_foregroundBlur_307_266"
            />
        </filter>
        <filter
            id="filter3_d_307_266"
            x="0.514648"
            y="0.829193"
            width="12.9707"
            height="12.9708"
            filterUnits="userSpaceOnUse"
            color-interpolation-filters="sRGB"
        >
            <feFlood flood-opacity="0" result="BackgroundImageFix" />
            <feColorMatrix
                in="SourceAlpha"
                type="matrix"
                values="0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 127 0"
                result="hardAlpha"
            />
            <feOffset dy="0.5" />
            <feGaussianBlur stdDeviation="0.5" />
            <feComposite in2="hardAlpha" operator="out" />
            <feColorMatrix
                type="matrix"
                values="0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 1 0"
            />
            <feBlend
                mode="normal"
                in2="BackgroundImageFix"
                result="effect1_dropShadow_307_266"
            />
            <feBlend
                mode="normal"
                in="SourceGraphic"
                in2="effect1_dropShadow_307_266"
                result="shape"
            />
        </filter>
        <linearGradient
            id="paint0_linear_307_266"
            x1="7"
            y1="6"
            x2="7"
            y2="10"
            gradientUnits="userSpaceOnUse"
        >
            <stop />
            <stop offset="1" stop-color="#444444" />
        </linearGradient>
        <clipPath id="clip0_307_266">
            <rect width="14" height="14" rx="2" fill="white" />
        </clipPath>
    </defs>
</svg>
