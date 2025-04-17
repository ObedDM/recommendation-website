import LoginBackground from "./components/Login/LoginBackground"
import NavBar from "./components/Login/LoginNavBar"
import LoginPanel from "./components/Login/LoginPanel"

function App() {

  const navItems = ['PRICING', 'CONTACT', 'TERMS OF USE', 'API', 'COOKIES']

  return (
    <>
      <main className="relative w-full h-screen">
        <LoginBackground />
        <LoginPanel
          buttonLogIn= {["Log in", "custom-login-gradient"]}
          buttonCreateAccount={["Create account", "custom-createAcc-gradient"]} />
      </main>

      <footer>
        <div className="fixed bottom-0 w-full">
          {<NavBar navItems={navItems} />}
        </div>
      </footer>
    </>
  )
}

export default App