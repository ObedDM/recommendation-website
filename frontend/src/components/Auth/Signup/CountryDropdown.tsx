import { useMemo, useState } from "react"
import Select from 'react-select'
import countryList, { CountryOption } from 'react-select-country-list'

interface CountryDropdownProp {
    onCountryChange: (country: CountryOption | null) => void;
}

export default function CountryDropdown({ onCountryChange }: CountryDropdownProp) {
    const options = useMemo(() => countryList().getData(), [])
    const[value, setValue] = useState<CountryOption | null>(null)

    function getCountry(country: CountryOption | null) {
        setValue(country)
        onCountryChange(country)
    }

    return (
        <Select
            styles={{
                control: (base, state) => ({
                    ...base,
                    outline: 'none',
                    boxShadow: state.isFocused ? '0 0 0 1px black' : undefined,
                    borderColor:  state.isFocused ? 'black' : '',
                    '&:hover': { borderColor: 'black' },
                    borderRadius: 5,
                }),

                menuList: (base) => ({
                    ...base,
                    maxHeight: '150px',
                    overflowY: 'auto',
                }),
            }}
            placeholder="Select a country"
            options={options}
            value={value}
            onChange={getCountry} />
    )
}