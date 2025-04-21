declare module 'react-select-country-list' {
    interface CountryOption {
        label: string
        value: string
    }

    interface CountryList {
        getData: () => CountryOption[]
    }

    export default function countryList(): CountryList
}