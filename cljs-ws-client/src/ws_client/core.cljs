(ns ws-client.core
  (:require [cljs-websockets.core :as ws]
            [cljs.core.async :refer [<! put! chan]]
            [clojure.data.json :as json])
  (:require-macros [cljs.core.async.macros :refer [go]]))

(defn on-open []
  (println "WebSocket connection established."))

(defn on-message [message]
  (println "Message received from server:" message))

(defn on-close []
  (println "WebSocket connection closed."))

(defn on-error [error]
  (println "WebSocket error:" error))

(defn send-message [socket]
  (let [msg {:action_type "join"
             :data {:friendly_name "cljs!"}}]
    (ws/send socket (json/write-str msg))))

(defn start []
  (let [socket (ws/connect "ws://localhost:8000"
                           {:on-open on-open
                            :on-message on-message
                            :on-close on-close
                            :on-error on-error})]
    ;; After connection is established, send a message
    (go
      (<! (on-open))
      (send-message socket))))

;; Call start function
(start)