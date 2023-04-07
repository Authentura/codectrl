module App exposing (..)

import Html exposing (..)
import Html.Attributes exposing (class)
import Html.Events exposing (onClick)


view : msg -> Html msg
view command =
    div []
        [ text "Hello"
        , button
            [ onClick command
            , class "bg-authenturaRed"
            , class "dark:bg-backgroundDark"
            , class "text-primaryDark"
            , class "rounded"
            , class "p-4"
            ]
            [ text "Say Hello!" ]
        ]
