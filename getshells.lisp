#!/usr/bin/env -S sbcl --script

(defun occurrences (lst)
  (declare (type simple-list lst)
           (optimize (speed 3) (debug 0) (safety 0)))
  (let ((table (make-hash-table :size (length lst) :test #'equal)))
    (loop for shell in lst do
      (let ((count (gethash shell table 0)))
        (setf (gethash shell table) (1+ count))))
    table))

(defun main ()
  (declare (optimize (speed 3) (debug 0) (safety 0)))
  (with-open-file (stream "passwd")
    (let ((shells (make-array 256 :initial-element nil)))
      (loop for line = (read-line stream nil)
            while line do
            (let* ((tokens (coerce (split-sequence:split-sequence #\: line) 'list))
                   (shell (if (>= (length tokens) 7) (nth 6 tokens) "")))
              (when (> (length shell) 0)
                (let ((idx (char-code (char shell 0))))
                  (unless (aref shells idx)
                    (setf (aref shells idx) (list shell)))
                  (push shell (aref shells idx))))))
      (loop for i below 256
            for shells = (aref shells i)
            when shells do
            (format t "~a~t~d~%" (car shells) (length shells))))))

(main)

