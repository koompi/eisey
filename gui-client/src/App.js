import { Switch, Route, Redirect } from "react-router-dom";
import Home from "./Home";
import SignUp from "./SignUp";

function App() {
  return (
    <div className="App">
      <Switch>
        <Route exact path="/" component={Home} />
        <Route exact path="/signup" component={SignUp} />
      </Switch>
    </div>
  );
}

export default App;
