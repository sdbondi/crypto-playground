port module Main exposing (Msg(..), main, update, view)

import Browser
import Html exposing (..)
import Html.Events exposing (onClick)

port increment : (Model, Int) -> Cmd a
port decrement : (Model, Int) -> Cmd a

type alias Model = 
  { name: String, counter: Int }

init: Maybe Model -> ( Model, Cmd Msg )
init model =
  ( Maybe.withDefault { name = "test", counter = 1 } model
  , Cmd.none
  )


type Msg
    = Increment
    | Decrement


update: Msg -> Model -> (Model, Cmd Msg) 
update msg model =
    case msg of
        Increment ->
            ({ model | counter = model.counter + 1}, increment(model, 1))

        Decrement ->
            ({ model | counter = model.counter - 1}, decrement(model, 1))


view: Model -> Html Msg
view model =
    div []
        [ button [ onClick Decrement ] [ text "-" ]
        , div [] [ text (model.name) ]
        , div [] [ text (String.fromInt model.counter) ]
        , button [ onClick Increment ] [ text "+" ]
        ]

main : Program (Maybe Model) Model Msg
main =
  Browser.document
    { init = init
    , view = \model -> { title = "Crypto playground", body = [view model] }
    , update = update
    , subscriptions = \_ -> Sub.none
    }