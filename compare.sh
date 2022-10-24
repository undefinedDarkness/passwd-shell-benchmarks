for i in getshells getshells.py getshells.pl getshells.sh getshells.ps1
do
  echo "################################################"
  echo $i
  time ./${i}
done

