import RootBackground from "./components/Other/Background/RootBackground"
import RootFrontMessage from "./components/Root/RootFrontMessage"
import RootNavBar from "./components/Root/RootNavBar"

function App() {

  const items = ["PRICING", "CONTACT", "TERMS OF USE", "API", "COOKIES"]

  return (
    <>
      <header>
        <RootNavBar navItems={items}/> 
      </header>

      <main>
        <RootBackground />
        <RootFrontMessage />
      </main>
    </>
  )
}

export default App