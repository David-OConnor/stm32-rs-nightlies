#[doc = "Register `DCKCFGR1` reader"]
pub struct R(crate::R<DCKCFGR1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DCKCFGR1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DCKCFGR1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DCKCFGR1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DCKCFGR1` writer"]
pub struct W(crate::W<DCKCFGR1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DCKCFGR1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<DCKCFGR1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DCKCFGR1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PLLI2SDIVQ` reader - PLLI2S division factor for SAI1 clock"]
pub type PLLI2SDIVQ_R = crate::FieldReader<u8, PLLI2SDIVQ_A>;
#[doc = "PLLI2S division factor for SAI1 clock\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PLLI2SDIVQ_A {
    #[doc = "0: PLLI2SDIVQ = /1"]
    Div1 = 0,
    #[doc = "1: PLLI2SDIVQ = /2"]
    Div2 = 1,
    #[doc = "2: PLLI2SDIVQ = /3"]
    Div3 = 2,
    #[doc = "3: PLLI2SDIVQ = /4"]
    Div4 = 3,
    #[doc = "4: PLLI2SDIVQ = /5"]
    Div5 = 4,
    #[doc = "5: PLLI2SDIVQ = /6"]
    Div6 = 5,
    #[doc = "6: PLLI2SDIVQ = /7"]
    Div7 = 6,
    #[doc = "7: PLLI2SDIVQ = /8"]
    Div8 = 7,
    #[doc = "8: PLLI2SDIVQ = /9"]
    Div9 = 8,
    #[doc = "9: PLLI2SDIVQ = /10"]
    Div10 = 9,
    #[doc = "10: PLLI2SDIVQ = /11"]
    Div11 = 10,
    #[doc = "11: PLLI2SDIVQ = /12"]
    Div12 = 11,
    #[doc = "12: PLLI2SDIVQ = /13"]
    Div13 = 12,
    #[doc = "13: PLLI2SDIVQ = /14"]
    Div14 = 13,
    #[doc = "14: PLLI2SDIVQ = /15"]
    Div15 = 14,
    #[doc = "15: PLLI2SDIVQ = /16"]
    Div16 = 15,
    #[doc = "16: PLLI2SDIVQ = /17"]
    Div17 = 16,
    #[doc = "17: PLLI2SDIVQ = /18"]
    Div18 = 17,
    #[doc = "18: PLLI2SDIVQ = /19"]
    Div19 = 18,
    #[doc = "19: PLLI2SDIVQ = /20"]
    Div20 = 19,
    #[doc = "20: PLLI2SDIVQ = /21"]
    Div21 = 20,
    #[doc = "21: PLLI2SDIVQ = /22"]
    Div22 = 21,
    #[doc = "22: PLLI2SDIVQ = /23"]
    Div23 = 22,
    #[doc = "23: PLLI2SDIVQ = /24"]
    Div24 = 23,
    #[doc = "24: PLLI2SDIVQ = /25"]
    Div25 = 24,
    #[doc = "25: PLLI2SDIVQ = /26"]
    Div26 = 25,
    #[doc = "26: PLLI2SDIVQ = /27"]
    Div27 = 26,
    #[doc = "27: PLLI2SDIVQ = /28"]
    Div28 = 27,
    #[doc = "28: PLLI2SDIVQ = /29"]
    Div29 = 28,
    #[doc = "29: PLLI2SDIVQ = /30"]
    Div30 = 29,
    #[doc = "30: PLLI2SDIVQ = /31"]
    Div31 = 30,
    #[doc = "31: PLLI2SDIVQ = /32"]
    Div32 = 31,
}
impl From<PLLI2SDIVQ_A> for u8 {
    #[inline(always)]
    fn from(variant: PLLI2SDIVQ_A) -> Self {
        variant as _
    }
}
impl PLLI2SDIVQ_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PLLI2SDIVQ_A {
        match self.bits {
            0 => PLLI2SDIVQ_A::Div1,
            1 => PLLI2SDIVQ_A::Div2,
            2 => PLLI2SDIVQ_A::Div3,
            3 => PLLI2SDIVQ_A::Div4,
            4 => PLLI2SDIVQ_A::Div5,
            5 => PLLI2SDIVQ_A::Div6,
            6 => PLLI2SDIVQ_A::Div7,
            7 => PLLI2SDIVQ_A::Div8,
            8 => PLLI2SDIVQ_A::Div9,
            9 => PLLI2SDIVQ_A::Div10,
            10 => PLLI2SDIVQ_A::Div11,
            11 => PLLI2SDIVQ_A::Div12,
            12 => PLLI2SDIVQ_A::Div13,
            13 => PLLI2SDIVQ_A::Div14,
            14 => PLLI2SDIVQ_A::Div15,
            15 => PLLI2SDIVQ_A::Div16,
            16 => PLLI2SDIVQ_A::Div17,
            17 => PLLI2SDIVQ_A::Div18,
            18 => PLLI2SDIVQ_A::Div19,
            19 => PLLI2SDIVQ_A::Div20,
            20 => PLLI2SDIVQ_A::Div21,
            21 => PLLI2SDIVQ_A::Div22,
            22 => PLLI2SDIVQ_A::Div23,
            23 => PLLI2SDIVQ_A::Div24,
            24 => PLLI2SDIVQ_A::Div25,
            25 => PLLI2SDIVQ_A::Div26,
            26 => PLLI2SDIVQ_A::Div27,
            27 => PLLI2SDIVQ_A::Div28,
            28 => PLLI2SDIVQ_A::Div29,
            29 => PLLI2SDIVQ_A::Div30,
            30 => PLLI2SDIVQ_A::Div31,
            31 => PLLI2SDIVQ_A::Div32,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `Div1`"]
    #[inline(always)]
    pub fn is_div1(&self) -> bool {
        *self == PLLI2SDIVQ_A::Div1
    }
    #[doc = "Checks if the value of the field is `Div2`"]
    #[inline(always)]
    pub fn is_div2(&self) -> bool {
        *self == PLLI2SDIVQ_A::Div2
    }
    #[doc = "Checks if the value of the field is `Div3`"]
    #[inline(always)]
    pub fn is_div3(&self) -> bool {
        *self == PLLI2SDIVQ_A::Div3
    }
    #[doc = "Checks if the value of the field is `Div4`"]
    #[inline(always)]
    pub fn is_div4(&self) -> bool {
        *self == PLLI2SDIVQ_A::Div4
    }
    #[doc = "Checks if the value of the field is `Div5`"]
    #[inline(always)]
    pub fn is_div5(&self) -> bool {
        *self == PLLI2SDIVQ_A::Div5
    }
    #[doc = "Checks if the value of the field is `Div6`"]
    #[inline(always)]
    pub fn is_div6(&self) -> bool {
        *self == PLLI2SDIVQ_A::Div6
    }
    #[doc = "Checks if the value of the field is `Div7`"]
    #[inline(always)]
    pub fn is_div7(&self) -> bool {
        *self == PLLI2SDIVQ_A::Div7
    }
    #[doc = "Checks if the value of the field is `Div8`"]
    #[inline(always)]
    pub fn is_div8(&self) -> bool {
        *self == PLLI2SDIVQ_A::Div8
    }
    #[doc = "Checks if the value of the field is `Div9`"]
    #[inline(always)]
    pub fn is_div9(&self) -> bool {
        *self == PLLI2SDIVQ_A::Div9
    }
    #[doc = "Checks if the value of the field is `Div10`"]
    #[inline(always)]
    pub fn is_div10(&self) -> bool {
        *self == PLLI2SDIVQ_A::Div10
    }
    #[doc = "Checks if the value of the field is `Div11`"]
    #[inline(always)]
    pub fn is_div11(&self) -> bool {
        *self == PLLI2SDIVQ_A::Div11
    }
    #[doc = "Checks if the value of the field is `Div12`"]
    #[inline(always)]
    pub fn is_div12(&self) -> bool {
        *self == PLLI2SDIVQ_A::Div12
    }
    #[doc = "Checks if the value of the field is `Div13`"]
    #[inline(always)]
    pub fn is_div13(&self) -> bool {
        *self == PLLI2SDIVQ_A::Div13
    }
    #[doc = "Checks if the value of the field is `Div14`"]
    #[inline(always)]
    pub fn is_div14(&self) -> bool {
        *self == PLLI2SDIVQ_A::Div14
    }
    #[doc = "Checks if the value of the field is `Div15`"]
    #[inline(always)]
    pub fn is_div15(&self) -> bool {
        *self == PLLI2SDIVQ_A::Div15
    }
    #[doc = "Checks if the value of the field is `Div16`"]
    #[inline(always)]
    pub fn is_div16(&self) -> bool {
        *self == PLLI2SDIVQ_A::Div16
    }
    #[doc = "Checks if the value of the field is `Div17`"]
    #[inline(always)]
    pub fn is_div17(&self) -> bool {
        *self == PLLI2SDIVQ_A::Div17
    }
    #[doc = "Checks if the value of the field is `Div18`"]
    #[inline(always)]
    pub fn is_div18(&self) -> bool {
        *self == PLLI2SDIVQ_A::Div18
    }
    #[doc = "Checks if the value of the field is `Div19`"]
    #[inline(always)]
    pub fn is_div19(&self) -> bool {
        *self == PLLI2SDIVQ_A::Div19
    }
    #[doc = "Checks if the value of the field is `Div20`"]
    #[inline(always)]
    pub fn is_div20(&self) -> bool {
        *self == PLLI2SDIVQ_A::Div20
    }
    #[doc = "Checks if the value of the field is `Div21`"]
    #[inline(always)]
    pub fn is_div21(&self) -> bool {
        *self == PLLI2SDIVQ_A::Div21
    }
    #[doc = "Checks if the value of the field is `Div22`"]
    #[inline(always)]
    pub fn is_div22(&self) -> bool {
        *self == PLLI2SDIVQ_A::Div22
    }
    #[doc = "Checks if the value of the field is `Div23`"]
    #[inline(always)]
    pub fn is_div23(&self) -> bool {
        *self == PLLI2SDIVQ_A::Div23
    }
    #[doc = "Checks if the value of the field is `Div24`"]
    #[inline(always)]
    pub fn is_div24(&self) -> bool {
        *self == PLLI2SDIVQ_A::Div24
    }
    #[doc = "Checks if the value of the field is `Div25`"]
    #[inline(always)]
    pub fn is_div25(&self) -> bool {
        *self == PLLI2SDIVQ_A::Div25
    }
    #[doc = "Checks if the value of the field is `Div26`"]
    #[inline(always)]
    pub fn is_div26(&self) -> bool {
        *self == PLLI2SDIVQ_A::Div26
    }
    #[doc = "Checks if the value of the field is `Div27`"]
    #[inline(always)]
    pub fn is_div27(&self) -> bool {
        *self == PLLI2SDIVQ_A::Div27
    }
    #[doc = "Checks if the value of the field is `Div28`"]
    #[inline(always)]
    pub fn is_div28(&self) -> bool {
        *self == PLLI2SDIVQ_A::Div28
    }
    #[doc = "Checks if the value of the field is `Div29`"]
    #[inline(always)]
    pub fn is_div29(&self) -> bool {
        *self == PLLI2SDIVQ_A::Div29
    }
    #[doc = "Checks if the value of the field is `Div30`"]
    #[inline(always)]
    pub fn is_div30(&self) -> bool {
        *self == PLLI2SDIVQ_A::Div30
    }
    #[doc = "Checks if the value of the field is `Div31`"]
    #[inline(always)]
    pub fn is_div31(&self) -> bool {
        *self == PLLI2SDIVQ_A::Div31
    }
    #[doc = "Checks if the value of the field is `Div32`"]
    #[inline(always)]
    pub fn is_div32(&self) -> bool {
        *self == PLLI2SDIVQ_A::Div32
    }
}
#[doc = "Field `PLLI2SDIVQ` writer - PLLI2S division factor for SAI1 clock"]
pub type PLLI2SDIVQ_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, DCKCFGR1_SPEC, u8, PLLI2SDIVQ_A, 5, O>;
impl<'a, const O: u8> PLLI2SDIVQ_W<'a, O> {
    #[doc = "PLLI2SDIVQ = /1"]
    #[inline(always)]
    pub fn div1(self) -> &'a mut W {
        self.variant(PLLI2SDIVQ_A::Div1)
    }
    #[doc = "PLLI2SDIVQ = /2"]
    #[inline(always)]
    pub fn div2(self) -> &'a mut W {
        self.variant(PLLI2SDIVQ_A::Div2)
    }
    #[doc = "PLLI2SDIVQ = /3"]
    #[inline(always)]
    pub fn div3(self) -> &'a mut W {
        self.variant(PLLI2SDIVQ_A::Div3)
    }
    #[doc = "PLLI2SDIVQ = /4"]
    #[inline(always)]
    pub fn div4(self) -> &'a mut W {
        self.variant(PLLI2SDIVQ_A::Div4)
    }
    #[doc = "PLLI2SDIVQ = /5"]
    #[inline(always)]
    pub fn div5(self) -> &'a mut W {
        self.variant(PLLI2SDIVQ_A::Div5)
    }
    #[doc = "PLLI2SDIVQ = /6"]
    #[inline(always)]
    pub fn div6(self) -> &'a mut W {
        self.variant(PLLI2SDIVQ_A::Div6)
    }
    #[doc = "PLLI2SDIVQ = /7"]
    #[inline(always)]
    pub fn div7(self) -> &'a mut W {
        self.variant(PLLI2SDIVQ_A::Div7)
    }
    #[doc = "PLLI2SDIVQ = /8"]
    #[inline(always)]
    pub fn div8(self) -> &'a mut W {
        self.variant(PLLI2SDIVQ_A::Div8)
    }
    #[doc = "PLLI2SDIVQ = /9"]
    #[inline(always)]
    pub fn div9(self) -> &'a mut W {
        self.variant(PLLI2SDIVQ_A::Div9)
    }
    #[doc = "PLLI2SDIVQ = /10"]
    #[inline(always)]
    pub fn div10(self) -> &'a mut W {
        self.variant(PLLI2SDIVQ_A::Div10)
    }
    #[doc = "PLLI2SDIVQ = /11"]
    #[inline(always)]
    pub fn div11(self) -> &'a mut W {
        self.variant(PLLI2SDIVQ_A::Div11)
    }
    #[doc = "PLLI2SDIVQ = /12"]
    #[inline(always)]
    pub fn div12(self) -> &'a mut W {
        self.variant(PLLI2SDIVQ_A::Div12)
    }
    #[doc = "PLLI2SDIVQ = /13"]
    #[inline(always)]
    pub fn div13(self) -> &'a mut W {
        self.variant(PLLI2SDIVQ_A::Div13)
    }
    #[doc = "PLLI2SDIVQ = /14"]
    #[inline(always)]
    pub fn div14(self) -> &'a mut W {
        self.variant(PLLI2SDIVQ_A::Div14)
    }
    #[doc = "PLLI2SDIVQ = /15"]
    #[inline(always)]
    pub fn div15(self) -> &'a mut W {
        self.variant(PLLI2SDIVQ_A::Div15)
    }
    #[doc = "PLLI2SDIVQ = /16"]
    #[inline(always)]
    pub fn div16(self) -> &'a mut W {
        self.variant(PLLI2SDIVQ_A::Div16)
    }
    #[doc = "PLLI2SDIVQ = /17"]
    #[inline(always)]
    pub fn div17(self) -> &'a mut W {
        self.variant(PLLI2SDIVQ_A::Div17)
    }
    #[doc = "PLLI2SDIVQ = /18"]
    #[inline(always)]
    pub fn div18(self) -> &'a mut W {
        self.variant(PLLI2SDIVQ_A::Div18)
    }
    #[doc = "PLLI2SDIVQ = /19"]
    #[inline(always)]
    pub fn div19(self) -> &'a mut W {
        self.variant(PLLI2SDIVQ_A::Div19)
    }
    #[doc = "PLLI2SDIVQ = /20"]
    #[inline(always)]
    pub fn div20(self) -> &'a mut W {
        self.variant(PLLI2SDIVQ_A::Div20)
    }
    #[doc = "PLLI2SDIVQ = /21"]
    #[inline(always)]
    pub fn div21(self) -> &'a mut W {
        self.variant(PLLI2SDIVQ_A::Div21)
    }
    #[doc = "PLLI2SDIVQ = /22"]
    #[inline(always)]
    pub fn div22(self) -> &'a mut W {
        self.variant(PLLI2SDIVQ_A::Div22)
    }
    #[doc = "PLLI2SDIVQ = /23"]
    #[inline(always)]
    pub fn div23(self) -> &'a mut W {
        self.variant(PLLI2SDIVQ_A::Div23)
    }
    #[doc = "PLLI2SDIVQ = /24"]
    #[inline(always)]
    pub fn div24(self) -> &'a mut W {
        self.variant(PLLI2SDIVQ_A::Div24)
    }
    #[doc = "PLLI2SDIVQ = /25"]
    #[inline(always)]
    pub fn div25(self) -> &'a mut W {
        self.variant(PLLI2SDIVQ_A::Div25)
    }
    #[doc = "PLLI2SDIVQ = /26"]
    #[inline(always)]
    pub fn div26(self) -> &'a mut W {
        self.variant(PLLI2SDIVQ_A::Div26)
    }
    #[doc = "PLLI2SDIVQ = /27"]
    #[inline(always)]
    pub fn div27(self) -> &'a mut W {
        self.variant(PLLI2SDIVQ_A::Div27)
    }
    #[doc = "PLLI2SDIVQ = /28"]
    #[inline(always)]
    pub fn div28(self) -> &'a mut W {
        self.variant(PLLI2SDIVQ_A::Div28)
    }
    #[doc = "PLLI2SDIVQ = /29"]
    #[inline(always)]
    pub fn div29(self) -> &'a mut W {
        self.variant(PLLI2SDIVQ_A::Div29)
    }
    #[doc = "PLLI2SDIVQ = /30"]
    #[inline(always)]
    pub fn div30(self) -> &'a mut W {
        self.variant(PLLI2SDIVQ_A::Div30)
    }
    #[doc = "PLLI2SDIVQ = /31"]
    #[inline(always)]
    pub fn div31(self) -> &'a mut W {
        self.variant(PLLI2SDIVQ_A::Div31)
    }
    #[doc = "PLLI2SDIVQ = /32"]
    #[inline(always)]
    pub fn div32(self) -> &'a mut W {
        self.variant(PLLI2SDIVQ_A::Div32)
    }
}
#[doc = "Field `PLLSAIDIVQ` reader - PLLSAI division factor for SAI1 clock"]
pub type PLLSAIDIVQ_R = crate::FieldReader<u8, PLLSAIDIVQ_A>;
#[doc = "PLLSAI division factor for SAI1 clock\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PLLSAIDIVQ_A {
    #[doc = "0: PLLSAIDIVQ = /1"]
    Div1 = 0,
    #[doc = "1: PLLSAIDIVQ = /2"]
    Div2 = 1,
    #[doc = "2: PLLSAIDIVQ = /3"]
    Div3 = 2,
    #[doc = "3: PLLSAIDIVQ = /4"]
    Div4 = 3,
    #[doc = "4: PLLSAIDIVQ = /5"]
    Div5 = 4,
    #[doc = "5: PLLSAIDIVQ = /6"]
    Div6 = 5,
    #[doc = "6: PLLSAIDIVQ = /7"]
    Div7 = 6,
    #[doc = "7: PLLSAIDIVQ = /8"]
    Div8 = 7,
    #[doc = "8: PLLSAIDIVQ = /9"]
    Div9 = 8,
    #[doc = "9: PLLSAIDIVQ = /10"]
    Div10 = 9,
    #[doc = "10: PLLSAIDIVQ = /11"]
    Div11 = 10,
    #[doc = "11: PLLSAIDIVQ = /12"]
    Div12 = 11,
    #[doc = "12: PLLSAIDIVQ = /13"]
    Div13 = 12,
    #[doc = "13: PLLSAIDIVQ = /14"]
    Div14 = 13,
    #[doc = "14: PLLSAIDIVQ = /15"]
    Div15 = 14,
    #[doc = "15: PLLSAIDIVQ = /16"]
    Div16 = 15,
    #[doc = "16: PLLSAIDIVQ = /17"]
    Div17 = 16,
    #[doc = "17: PLLSAIDIVQ = /18"]
    Div18 = 17,
    #[doc = "18: PLLSAIDIVQ = /19"]
    Div19 = 18,
    #[doc = "19: PLLSAIDIVQ = /20"]
    Div20 = 19,
    #[doc = "20: PLLSAIDIVQ = /21"]
    Div21 = 20,
    #[doc = "21: PLLSAIDIVQ = /22"]
    Div22 = 21,
    #[doc = "22: PLLSAIDIVQ = /23"]
    Div23 = 22,
    #[doc = "23: PLLSAIDIVQ = /24"]
    Div24 = 23,
    #[doc = "24: PLLSAIDIVQ = /25"]
    Div25 = 24,
    #[doc = "25: PLLSAIDIVQ = /26"]
    Div26 = 25,
    #[doc = "26: PLLSAIDIVQ = /27"]
    Div27 = 26,
    #[doc = "27: PLLSAIDIVQ = /28"]
    Div28 = 27,
    #[doc = "28: PLLSAIDIVQ = /29"]
    Div29 = 28,
    #[doc = "29: PLLSAIDIVQ = /30"]
    Div30 = 29,
    #[doc = "30: PLLSAIDIVQ = /31"]
    Div31 = 30,
    #[doc = "31: PLLSAIDIVQ = /32"]
    Div32 = 31,
}
impl From<PLLSAIDIVQ_A> for u8 {
    #[inline(always)]
    fn from(variant: PLLSAIDIVQ_A) -> Self {
        variant as _
    }
}
impl PLLSAIDIVQ_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PLLSAIDIVQ_A {
        match self.bits {
            0 => PLLSAIDIVQ_A::Div1,
            1 => PLLSAIDIVQ_A::Div2,
            2 => PLLSAIDIVQ_A::Div3,
            3 => PLLSAIDIVQ_A::Div4,
            4 => PLLSAIDIVQ_A::Div5,
            5 => PLLSAIDIVQ_A::Div6,
            6 => PLLSAIDIVQ_A::Div7,
            7 => PLLSAIDIVQ_A::Div8,
            8 => PLLSAIDIVQ_A::Div9,
            9 => PLLSAIDIVQ_A::Div10,
            10 => PLLSAIDIVQ_A::Div11,
            11 => PLLSAIDIVQ_A::Div12,
            12 => PLLSAIDIVQ_A::Div13,
            13 => PLLSAIDIVQ_A::Div14,
            14 => PLLSAIDIVQ_A::Div15,
            15 => PLLSAIDIVQ_A::Div16,
            16 => PLLSAIDIVQ_A::Div17,
            17 => PLLSAIDIVQ_A::Div18,
            18 => PLLSAIDIVQ_A::Div19,
            19 => PLLSAIDIVQ_A::Div20,
            20 => PLLSAIDIVQ_A::Div21,
            21 => PLLSAIDIVQ_A::Div22,
            22 => PLLSAIDIVQ_A::Div23,
            23 => PLLSAIDIVQ_A::Div24,
            24 => PLLSAIDIVQ_A::Div25,
            25 => PLLSAIDIVQ_A::Div26,
            26 => PLLSAIDIVQ_A::Div27,
            27 => PLLSAIDIVQ_A::Div28,
            28 => PLLSAIDIVQ_A::Div29,
            29 => PLLSAIDIVQ_A::Div30,
            30 => PLLSAIDIVQ_A::Div31,
            31 => PLLSAIDIVQ_A::Div32,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `Div1`"]
    #[inline(always)]
    pub fn is_div1(&self) -> bool {
        *self == PLLSAIDIVQ_A::Div1
    }
    #[doc = "Checks if the value of the field is `Div2`"]
    #[inline(always)]
    pub fn is_div2(&self) -> bool {
        *self == PLLSAIDIVQ_A::Div2
    }
    #[doc = "Checks if the value of the field is `Div3`"]
    #[inline(always)]
    pub fn is_div3(&self) -> bool {
        *self == PLLSAIDIVQ_A::Div3
    }
    #[doc = "Checks if the value of the field is `Div4`"]
    #[inline(always)]
    pub fn is_div4(&self) -> bool {
        *self == PLLSAIDIVQ_A::Div4
    }
    #[doc = "Checks if the value of the field is `Div5`"]
    #[inline(always)]
    pub fn is_div5(&self) -> bool {
        *self == PLLSAIDIVQ_A::Div5
    }
    #[doc = "Checks if the value of the field is `Div6`"]
    #[inline(always)]
    pub fn is_div6(&self) -> bool {
        *self == PLLSAIDIVQ_A::Div6
    }
    #[doc = "Checks if the value of the field is `Div7`"]
    #[inline(always)]
    pub fn is_div7(&self) -> bool {
        *self == PLLSAIDIVQ_A::Div7
    }
    #[doc = "Checks if the value of the field is `Div8`"]
    #[inline(always)]
    pub fn is_div8(&self) -> bool {
        *self == PLLSAIDIVQ_A::Div8
    }
    #[doc = "Checks if the value of the field is `Div9`"]
    #[inline(always)]
    pub fn is_div9(&self) -> bool {
        *self == PLLSAIDIVQ_A::Div9
    }
    #[doc = "Checks if the value of the field is `Div10`"]
    #[inline(always)]
    pub fn is_div10(&self) -> bool {
        *self == PLLSAIDIVQ_A::Div10
    }
    #[doc = "Checks if the value of the field is `Div11`"]
    #[inline(always)]
    pub fn is_div11(&self) -> bool {
        *self == PLLSAIDIVQ_A::Div11
    }
    #[doc = "Checks if the value of the field is `Div12`"]
    #[inline(always)]
    pub fn is_div12(&self) -> bool {
        *self == PLLSAIDIVQ_A::Div12
    }
    #[doc = "Checks if the value of the field is `Div13`"]
    #[inline(always)]
    pub fn is_div13(&self) -> bool {
        *self == PLLSAIDIVQ_A::Div13
    }
    #[doc = "Checks if the value of the field is `Div14`"]
    #[inline(always)]
    pub fn is_div14(&self) -> bool {
        *self == PLLSAIDIVQ_A::Div14
    }
    #[doc = "Checks if the value of the field is `Div15`"]
    #[inline(always)]
    pub fn is_div15(&self) -> bool {
        *self == PLLSAIDIVQ_A::Div15
    }
    #[doc = "Checks if the value of the field is `Div16`"]
    #[inline(always)]
    pub fn is_div16(&self) -> bool {
        *self == PLLSAIDIVQ_A::Div16
    }
    #[doc = "Checks if the value of the field is `Div17`"]
    #[inline(always)]
    pub fn is_div17(&self) -> bool {
        *self == PLLSAIDIVQ_A::Div17
    }
    #[doc = "Checks if the value of the field is `Div18`"]
    #[inline(always)]
    pub fn is_div18(&self) -> bool {
        *self == PLLSAIDIVQ_A::Div18
    }
    #[doc = "Checks if the value of the field is `Div19`"]
    #[inline(always)]
    pub fn is_div19(&self) -> bool {
        *self == PLLSAIDIVQ_A::Div19
    }
    #[doc = "Checks if the value of the field is `Div20`"]
    #[inline(always)]
    pub fn is_div20(&self) -> bool {
        *self == PLLSAIDIVQ_A::Div20
    }
    #[doc = "Checks if the value of the field is `Div21`"]
    #[inline(always)]
    pub fn is_div21(&self) -> bool {
        *self == PLLSAIDIVQ_A::Div21
    }
    #[doc = "Checks if the value of the field is `Div22`"]
    #[inline(always)]
    pub fn is_div22(&self) -> bool {
        *self == PLLSAIDIVQ_A::Div22
    }
    #[doc = "Checks if the value of the field is `Div23`"]
    #[inline(always)]
    pub fn is_div23(&self) -> bool {
        *self == PLLSAIDIVQ_A::Div23
    }
    #[doc = "Checks if the value of the field is `Div24`"]
    #[inline(always)]
    pub fn is_div24(&self) -> bool {
        *self == PLLSAIDIVQ_A::Div24
    }
    #[doc = "Checks if the value of the field is `Div25`"]
    #[inline(always)]
    pub fn is_div25(&self) -> bool {
        *self == PLLSAIDIVQ_A::Div25
    }
    #[doc = "Checks if the value of the field is `Div26`"]
    #[inline(always)]
    pub fn is_div26(&self) -> bool {
        *self == PLLSAIDIVQ_A::Div26
    }
    #[doc = "Checks if the value of the field is `Div27`"]
    #[inline(always)]
    pub fn is_div27(&self) -> bool {
        *self == PLLSAIDIVQ_A::Div27
    }
    #[doc = "Checks if the value of the field is `Div28`"]
    #[inline(always)]
    pub fn is_div28(&self) -> bool {
        *self == PLLSAIDIVQ_A::Div28
    }
    #[doc = "Checks if the value of the field is `Div29`"]
    #[inline(always)]
    pub fn is_div29(&self) -> bool {
        *self == PLLSAIDIVQ_A::Div29
    }
    #[doc = "Checks if the value of the field is `Div30`"]
    #[inline(always)]
    pub fn is_div30(&self) -> bool {
        *self == PLLSAIDIVQ_A::Div30
    }
    #[doc = "Checks if the value of the field is `Div31`"]
    #[inline(always)]
    pub fn is_div31(&self) -> bool {
        *self == PLLSAIDIVQ_A::Div31
    }
    #[doc = "Checks if the value of the field is `Div32`"]
    #[inline(always)]
    pub fn is_div32(&self) -> bool {
        *self == PLLSAIDIVQ_A::Div32
    }
}
#[doc = "Field `PLLSAIDIVQ` writer - PLLSAI division factor for SAI1 clock"]
pub type PLLSAIDIVQ_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, DCKCFGR1_SPEC, u8, PLLSAIDIVQ_A, 5, O>;
impl<'a, const O: u8> PLLSAIDIVQ_W<'a, O> {
    #[doc = "PLLSAIDIVQ = /1"]
    #[inline(always)]
    pub fn div1(self) -> &'a mut W {
        self.variant(PLLSAIDIVQ_A::Div1)
    }
    #[doc = "PLLSAIDIVQ = /2"]
    #[inline(always)]
    pub fn div2(self) -> &'a mut W {
        self.variant(PLLSAIDIVQ_A::Div2)
    }
    #[doc = "PLLSAIDIVQ = /3"]
    #[inline(always)]
    pub fn div3(self) -> &'a mut W {
        self.variant(PLLSAIDIVQ_A::Div3)
    }
    #[doc = "PLLSAIDIVQ = /4"]
    #[inline(always)]
    pub fn div4(self) -> &'a mut W {
        self.variant(PLLSAIDIVQ_A::Div4)
    }
    #[doc = "PLLSAIDIVQ = /5"]
    #[inline(always)]
    pub fn div5(self) -> &'a mut W {
        self.variant(PLLSAIDIVQ_A::Div5)
    }
    #[doc = "PLLSAIDIVQ = /6"]
    #[inline(always)]
    pub fn div6(self) -> &'a mut W {
        self.variant(PLLSAIDIVQ_A::Div6)
    }
    #[doc = "PLLSAIDIVQ = /7"]
    #[inline(always)]
    pub fn div7(self) -> &'a mut W {
        self.variant(PLLSAIDIVQ_A::Div7)
    }
    #[doc = "PLLSAIDIVQ = /8"]
    #[inline(always)]
    pub fn div8(self) -> &'a mut W {
        self.variant(PLLSAIDIVQ_A::Div8)
    }
    #[doc = "PLLSAIDIVQ = /9"]
    #[inline(always)]
    pub fn div9(self) -> &'a mut W {
        self.variant(PLLSAIDIVQ_A::Div9)
    }
    #[doc = "PLLSAIDIVQ = /10"]
    #[inline(always)]
    pub fn div10(self) -> &'a mut W {
        self.variant(PLLSAIDIVQ_A::Div10)
    }
    #[doc = "PLLSAIDIVQ = /11"]
    #[inline(always)]
    pub fn div11(self) -> &'a mut W {
        self.variant(PLLSAIDIVQ_A::Div11)
    }
    #[doc = "PLLSAIDIVQ = /12"]
    #[inline(always)]
    pub fn div12(self) -> &'a mut W {
        self.variant(PLLSAIDIVQ_A::Div12)
    }
    #[doc = "PLLSAIDIVQ = /13"]
    #[inline(always)]
    pub fn div13(self) -> &'a mut W {
        self.variant(PLLSAIDIVQ_A::Div13)
    }
    #[doc = "PLLSAIDIVQ = /14"]
    #[inline(always)]
    pub fn div14(self) -> &'a mut W {
        self.variant(PLLSAIDIVQ_A::Div14)
    }
    #[doc = "PLLSAIDIVQ = /15"]
    #[inline(always)]
    pub fn div15(self) -> &'a mut W {
        self.variant(PLLSAIDIVQ_A::Div15)
    }
    #[doc = "PLLSAIDIVQ = /16"]
    #[inline(always)]
    pub fn div16(self) -> &'a mut W {
        self.variant(PLLSAIDIVQ_A::Div16)
    }
    #[doc = "PLLSAIDIVQ = /17"]
    #[inline(always)]
    pub fn div17(self) -> &'a mut W {
        self.variant(PLLSAIDIVQ_A::Div17)
    }
    #[doc = "PLLSAIDIVQ = /18"]
    #[inline(always)]
    pub fn div18(self) -> &'a mut W {
        self.variant(PLLSAIDIVQ_A::Div18)
    }
    #[doc = "PLLSAIDIVQ = /19"]
    #[inline(always)]
    pub fn div19(self) -> &'a mut W {
        self.variant(PLLSAIDIVQ_A::Div19)
    }
    #[doc = "PLLSAIDIVQ = /20"]
    #[inline(always)]
    pub fn div20(self) -> &'a mut W {
        self.variant(PLLSAIDIVQ_A::Div20)
    }
    #[doc = "PLLSAIDIVQ = /21"]
    #[inline(always)]
    pub fn div21(self) -> &'a mut W {
        self.variant(PLLSAIDIVQ_A::Div21)
    }
    #[doc = "PLLSAIDIVQ = /22"]
    #[inline(always)]
    pub fn div22(self) -> &'a mut W {
        self.variant(PLLSAIDIVQ_A::Div22)
    }
    #[doc = "PLLSAIDIVQ = /23"]
    #[inline(always)]
    pub fn div23(self) -> &'a mut W {
        self.variant(PLLSAIDIVQ_A::Div23)
    }
    #[doc = "PLLSAIDIVQ = /24"]
    #[inline(always)]
    pub fn div24(self) -> &'a mut W {
        self.variant(PLLSAIDIVQ_A::Div24)
    }
    #[doc = "PLLSAIDIVQ = /25"]
    #[inline(always)]
    pub fn div25(self) -> &'a mut W {
        self.variant(PLLSAIDIVQ_A::Div25)
    }
    #[doc = "PLLSAIDIVQ = /26"]
    #[inline(always)]
    pub fn div26(self) -> &'a mut W {
        self.variant(PLLSAIDIVQ_A::Div26)
    }
    #[doc = "PLLSAIDIVQ = /27"]
    #[inline(always)]
    pub fn div27(self) -> &'a mut W {
        self.variant(PLLSAIDIVQ_A::Div27)
    }
    #[doc = "PLLSAIDIVQ = /28"]
    #[inline(always)]
    pub fn div28(self) -> &'a mut W {
        self.variant(PLLSAIDIVQ_A::Div28)
    }
    #[doc = "PLLSAIDIVQ = /29"]
    #[inline(always)]
    pub fn div29(self) -> &'a mut W {
        self.variant(PLLSAIDIVQ_A::Div29)
    }
    #[doc = "PLLSAIDIVQ = /30"]
    #[inline(always)]
    pub fn div30(self) -> &'a mut W {
        self.variant(PLLSAIDIVQ_A::Div30)
    }
    #[doc = "PLLSAIDIVQ = /31"]
    #[inline(always)]
    pub fn div31(self) -> &'a mut W {
        self.variant(PLLSAIDIVQ_A::Div31)
    }
    #[doc = "PLLSAIDIVQ = /32"]
    #[inline(always)]
    pub fn div32(self) -> &'a mut W {
        self.variant(PLLSAIDIVQ_A::Div32)
    }
}
#[doc = "Field `PLLSAIDIVR` reader - division factor for LCD_CLK"]
pub type PLLSAIDIVR_R = crate::FieldReader<u8, PLLSAIDIVR_A>;
#[doc = "division factor for LCD_CLK\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PLLSAIDIVR_A {
    #[doc = "0: PLLSAIDIVR = /2"]
    Div2 = 0,
    #[doc = "1: PLLSAIDIVR = /4"]
    Div4 = 1,
    #[doc = "2: PLLSAIDIVR = /8"]
    Div8 = 2,
    #[doc = "3: PLLSAIDIVR = /16"]
    Div16 = 3,
}
impl From<PLLSAIDIVR_A> for u8 {
    #[inline(always)]
    fn from(variant: PLLSAIDIVR_A) -> Self {
        variant as _
    }
}
impl PLLSAIDIVR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PLLSAIDIVR_A {
        match self.bits {
            0 => PLLSAIDIVR_A::Div2,
            1 => PLLSAIDIVR_A::Div4,
            2 => PLLSAIDIVR_A::Div8,
            3 => PLLSAIDIVR_A::Div16,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `Div2`"]
    #[inline(always)]
    pub fn is_div2(&self) -> bool {
        *self == PLLSAIDIVR_A::Div2
    }
    #[doc = "Checks if the value of the field is `Div4`"]
    #[inline(always)]
    pub fn is_div4(&self) -> bool {
        *self == PLLSAIDIVR_A::Div4
    }
    #[doc = "Checks if the value of the field is `Div8`"]
    #[inline(always)]
    pub fn is_div8(&self) -> bool {
        *self == PLLSAIDIVR_A::Div8
    }
    #[doc = "Checks if the value of the field is `Div16`"]
    #[inline(always)]
    pub fn is_div16(&self) -> bool {
        *self == PLLSAIDIVR_A::Div16
    }
}
#[doc = "Field `PLLSAIDIVR` writer - division factor for LCD_CLK"]
pub type PLLSAIDIVR_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, DCKCFGR1_SPEC, u8, PLLSAIDIVR_A, 2, O>;
impl<'a, const O: u8> PLLSAIDIVR_W<'a, O> {
    #[doc = "PLLSAIDIVR = /2"]
    #[inline(always)]
    pub fn div2(self) -> &'a mut W {
        self.variant(PLLSAIDIVR_A::Div2)
    }
    #[doc = "PLLSAIDIVR = /4"]
    #[inline(always)]
    pub fn div4(self) -> &'a mut W {
        self.variant(PLLSAIDIVR_A::Div4)
    }
    #[doc = "PLLSAIDIVR = /8"]
    #[inline(always)]
    pub fn div8(self) -> &'a mut W {
        self.variant(PLLSAIDIVR_A::Div8)
    }
    #[doc = "PLLSAIDIVR = /16"]
    #[inline(always)]
    pub fn div16(self) -> &'a mut W {
        self.variant(PLLSAIDIVR_A::Div16)
    }
}
#[doc = "Field `SAI1SEL` reader - SAI1 clock source selection"]
pub type SAI1SEL_R = crate::FieldReader<u8, SAI1SEL_A>;
#[doc = "SAI1 clock source selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SAI1SEL_A {
    #[doc = "0: SAI1 clock frequency = f(PLLSAI_Q) / PLLSAIDIVQ"]
    Pllsai = 0,
    #[doc = "1: SAI1 clock frequency = f(PLLI2S_Q) / PLLI2SDIVQ"]
    Plli2s = 1,
    #[doc = "2: SAI1 clock frequency = Alternate function input frequency"]
    Afif = 2,
    #[doc = "3: SAI1 clock frequency = HSI or HSE"]
    HsiHse = 3,
}
impl From<SAI1SEL_A> for u8 {
    #[inline(always)]
    fn from(variant: SAI1SEL_A) -> Self {
        variant as _
    }
}
impl SAI1SEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SAI1SEL_A {
        match self.bits {
            0 => SAI1SEL_A::Pllsai,
            1 => SAI1SEL_A::Plli2s,
            2 => SAI1SEL_A::Afif,
            3 => SAI1SEL_A::HsiHse,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `Pllsai`"]
    #[inline(always)]
    pub fn is_pllsai(&self) -> bool {
        *self == SAI1SEL_A::Pllsai
    }
    #[doc = "Checks if the value of the field is `Plli2s`"]
    #[inline(always)]
    pub fn is_plli2s(&self) -> bool {
        *self == SAI1SEL_A::Plli2s
    }
    #[doc = "Checks if the value of the field is `Afif`"]
    #[inline(always)]
    pub fn is_afif(&self) -> bool {
        *self == SAI1SEL_A::Afif
    }
    #[doc = "Checks if the value of the field is `HsiHse`"]
    #[inline(always)]
    pub fn is_hsi_hse(&self) -> bool {
        *self == SAI1SEL_A::HsiHse
    }
}
#[doc = "Field `SAI1SEL` writer - SAI1 clock source selection"]
pub type SAI1SEL_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, DCKCFGR1_SPEC, u8, SAI1SEL_A, 2, O>;
impl<'a, const O: u8> SAI1SEL_W<'a, O> {
    #[doc = "SAI1 clock frequency = f(PLLSAI_Q) / PLLSAIDIVQ"]
    #[inline(always)]
    pub fn pllsai(self) -> &'a mut W {
        self.variant(SAI1SEL_A::Pllsai)
    }
    #[doc = "SAI1 clock frequency = f(PLLI2S_Q) / PLLI2SDIVQ"]
    #[inline(always)]
    pub fn plli2s(self) -> &'a mut W {
        self.variant(SAI1SEL_A::Plli2s)
    }
    #[doc = "SAI1 clock frequency = Alternate function input frequency"]
    #[inline(always)]
    pub fn afif(self) -> &'a mut W {
        self.variant(SAI1SEL_A::Afif)
    }
    #[doc = "SAI1 clock frequency = HSI or HSE"]
    #[inline(always)]
    pub fn hsi_hse(self) -> &'a mut W {
        self.variant(SAI1SEL_A::HsiHse)
    }
}
#[doc = "Field `SAI2SEL` reader - SAI2 clock source selection"]
pub type SAI2SEL_R = crate::FieldReader<u8, SAI2SEL_A>;
#[doc = "SAI2 clock source selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SAI2SEL_A {
    #[doc = "0: SAI2 clock frequency = f(PLLSAI_Q) / PLLSAIDIVQ"]
    Pllsai = 0,
    #[doc = "1: SAI2 clock frequency = f(PLLI2S_Q) / PLLI2SDIVQ"]
    Plli2s = 1,
    #[doc = "2: SAI2 clock frequency = Alternate function input frequency"]
    Afif = 2,
    #[doc = "3: SAI2 clock frequency = HSI or HSE"]
    HsiHse = 3,
}
impl From<SAI2SEL_A> for u8 {
    #[inline(always)]
    fn from(variant: SAI2SEL_A) -> Self {
        variant as _
    }
}
impl SAI2SEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SAI2SEL_A {
        match self.bits {
            0 => SAI2SEL_A::Pllsai,
            1 => SAI2SEL_A::Plli2s,
            2 => SAI2SEL_A::Afif,
            3 => SAI2SEL_A::HsiHse,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `Pllsai`"]
    #[inline(always)]
    pub fn is_pllsai(&self) -> bool {
        *self == SAI2SEL_A::Pllsai
    }
    #[doc = "Checks if the value of the field is `Plli2s`"]
    #[inline(always)]
    pub fn is_plli2s(&self) -> bool {
        *self == SAI2SEL_A::Plli2s
    }
    #[doc = "Checks if the value of the field is `Afif`"]
    #[inline(always)]
    pub fn is_afif(&self) -> bool {
        *self == SAI2SEL_A::Afif
    }
    #[doc = "Checks if the value of the field is `HsiHse`"]
    #[inline(always)]
    pub fn is_hsi_hse(&self) -> bool {
        *self == SAI2SEL_A::HsiHse
    }
}
#[doc = "Field `SAI2SEL` writer - SAI2 clock source selection"]
pub type SAI2SEL_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, DCKCFGR1_SPEC, u8, SAI2SEL_A, 2, O>;
impl<'a, const O: u8> SAI2SEL_W<'a, O> {
    #[doc = "SAI2 clock frequency = f(PLLSAI_Q) / PLLSAIDIVQ"]
    #[inline(always)]
    pub fn pllsai(self) -> &'a mut W {
        self.variant(SAI2SEL_A::Pllsai)
    }
    #[doc = "SAI2 clock frequency = f(PLLI2S_Q) / PLLI2SDIVQ"]
    #[inline(always)]
    pub fn plli2s(self) -> &'a mut W {
        self.variant(SAI2SEL_A::Plli2s)
    }
    #[doc = "SAI2 clock frequency = Alternate function input frequency"]
    #[inline(always)]
    pub fn afif(self) -> &'a mut W {
        self.variant(SAI2SEL_A::Afif)
    }
    #[doc = "SAI2 clock frequency = HSI or HSE"]
    #[inline(always)]
    pub fn hsi_hse(self) -> &'a mut W {
        self.variant(SAI2SEL_A::HsiHse)
    }
}
#[doc = "Field `TIMPRE` reader - Timers clocks prescalers selection"]
pub type TIMPRE_R = crate::BitReader<TIMPRE_A>;
#[doc = "Timers clocks prescalers selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TIMPRE_A {
    #[doc = "0: If the APB prescaler is configured 1, TIMxCLK = PCLKx. Otherwise, TIMxCLK = 2xPCLKx"]
    Mul2 = 0,
    #[doc = "1: If the APB prescaler is configured 1, 2 or 4, TIMxCLK = HCLK. Otherwise, TIMxCLK = 4xPCLKx"]
    Mul4 = 1,
}
impl From<TIMPRE_A> for bool {
    #[inline(always)]
    fn from(variant: TIMPRE_A) -> Self {
        variant as u8 != 0
    }
}
impl TIMPRE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TIMPRE_A {
        match self.bits {
            false => TIMPRE_A::Mul2,
            true => TIMPRE_A::Mul4,
        }
    }
    #[doc = "Checks if the value of the field is `Mul2`"]
    #[inline(always)]
    pub fn is_mul2(&self) -> bool {
        *self == TIMPRE_A::Mul2
    }
    #[doc = "Checks if the value of the field is `Mul4`"]
    #[inline(always)]
    pub fn is_mul4(&self) -> bool {
        *self == TIMPRE_A::Mul4
    }
}
#[doc = "Field `TIMPRE` writer - Timers clocks prescalers selection"]
pub type TIMPRE_W<'a, const O: u8> = crate::BitWriter<'a, u32, DCKCFGR1_SPEC, TIMPRE_A, O>;
impl<'a, const O: u8> TIMPRE_W<'a, O> {
    #[doc = "If the APB prescaler is configured 1, TIMxCLK = PCLKx. Otherwise, TIMxCLK = 2xPCLKx"]
    #[inline(always)]
    pub fn mul2(self) -> &'a mut W {
        self.variant(TIMPRE_A::Mul2)
    }
    #[doc = "If the APB prescaler is configured 1, 2 or 4, TIMxCLK = HCLK. Otherwise, TIMxCLK = 4xPCLKx"]
    #[inline(always)]
    pub fn mul4(self) -> &'a mut W {
        self.variant(TIMPRE_A::Mul4)
    }
}
#[doc = "Field `DFSDM1SEL` reader - DFSDM1 clock source selection"]
pub type DFSDM1SEL_R = crate::BitReader<DFSDM1SEL_A>;
#[doc = "DFSDM1 clock source selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DFSDM1SEL_A {
    #[doc = "0: APB2 clock (PCLK2) selected as DFSDM1 Kernel clock source"]
    Apb2 = 0,
    #[doc = "1: System clock (SYSCLK) clock selected as DFSDM1 Kernel clock source"]
    Sysclk = 1,
}
impl From<DFSDM1SEL_A> for bool {
    #[inline(always)]
    fn from(variant: DFSDM1SEL_A) -> Self {
        variant as u8 != 0
    }
}
impl DFSDM1SEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DFSDM1SEL_A {
        match self.bits {
            false => DFSDM1SEL_A::Apb2,
            true => DFSDM1SEL_A::Sysclk,
        }
    }
    #[doc = "Checks if the value of the field is `Apb2`"]
    #[inline(always)]
    pub fn is_apb2(&self) -> bool {
        *self == DFSDM1SEL_A::Apb2
    }
    #[doc = "Checks if the value of the field is `Sysclk`"]
    #[inline(always)]
    pub fn is_sysclk(&self) -> bool {
        *self == DFSDM1SEL_A::Sysclk
    }
}
#[doc = "Field `DFSDM1SEL` writer - DFSDM1 clock source selection"]
pub type DFSDM1SEL_W<'a, const O: u8> = crate::BitWriter<'a, u32, DCKCFGR1_SPEC, DFSDM1SEL_A, O>;
impl<'a, const O: u8> DFSDM1SEL_W<'a, O> {
    #[doc = "APB2 clock (PCLK2) selected as DFSDM1 Kernel clock source"]
    #[inline(always)]
    pub fn apb2(self) -> &'a mut W {
        self.variant(DFSDM1SEL_A::Apb2)
    }
    #[doc = "System clock (SYSCLK) clock selected as DFSDM1 Kernel clock source"]
    #[inline(always)]
    pub fn sysclk(self) -> &'a mut W {
        self.variant(DFSDM1SEL_A::Sysclk)
    }
}
#[doc = "Field `ADFSDM1SEL` reader - DFSDM1 AUDIO clock source selection"]
pub type ADFSDM1SEL_R = crate::BitReader<ADFSDM1SEL_A>;
#[doc = "DFSDM1 AUDIO clock source selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ADFSDM1SEL_A {
    #[doc = "0: SAI1 clock selected as DFSDM1 Audio clock source"]
    Sai1 = 0,
    #[doc = "1: SAI2 clock selected as DFSDM1 Audio clock source"]
    Sai2 = 1,
}
impl From<ADFSDM1SEL_A> for bool {
    #[inline(always)]
    fn from(variant: ADFSDM1SEL_A) -> Self {
        variant as u8 != 0
    }
}
impl ADFSDM1SEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ADFSDM1SEL_A {
        match self.bits {
            false => ADFSDM1SEL_A::Sai1,
            true => ADFSDM1SEL_A::Sai2,
        }
    }
    #[doc = "Checks if the value of the field is `Sai1`"]
    #[inline(always)]
    pub fn is_sai1(&self) -> bool {
        *self == ADFSDM1SEL_A::Sai1
    }
    #[doc = "Checks if the value of the field is `Sai2`"]
    #[inline(always)]
    pub fn is_sai2(&self) -> bool {
        *self == ADFSDM1SEL_A::Sai2
    }
}
#[doc = "Field `ADFSDM1SEL` writer - DFSDM1 AUDIO clock source selection"]
pub type ADFSDM1SEL_W<'a, const O: u8> = crate::BitWriter<'a, u32, DCKCFGR1_SPEC, ADFSDM1SEL_A, O>;
impl<'a, const O: u8> ADFSDM1SEL_W<'a, O> {
    #[doc = "SAI1 clock selected as DFSDM1 Audio clock source"]
    #[inline(always)]
    pub fn sai1(self) -> &'a mut W {
        self.variant(ADFSDM1SEL_A::Sai1)
    }
    #[doc = "SAI2 clock selected as DFSDM1 Audio clock source"]
    #[inline(always)]
    pub fn sai2(self) -> &'a mut W {
        self.variant(ADFSDM1SEL_A::Sai2)
    }
}
impl R {
    #[doc = "Bits 0:4 - PLLI2S division factor for SAI1 clock"]
    #[inline(always)]
    pub fn plli2sdivq(&self) -> PLLI2SDIVQ_R {
        PLLI2SDIVQ_R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 8:12 - PLLSAI division factor for SAI1 clock"]
    #[inline(always)]
    pub fn pllsaidivq(&self) -> PLLSAIDIVQ_R {
        PLLSAIDIVQ_R::new(((self.bits >> 8) & 0x1f) as u8)
    }
    #[doc = "Bits 16:17 - division factor for LCD_CLK"]
    #[inline(always)]
    pub fn pllsaidivr(&self) -> PLLSAIDIVR_R {
        PLLSAIDIVR_R::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bits 20:21 - SAI1 clock source selection"]
    #[inline(always)]
    pub fn sai1sel(&self) -> SAI1SEL_R {
        SAI1SEL_R::new(((self.bits >> 20) & 3) as u8)
    }
    #[doc = "Bits 22:23 - SAI2 clock source selection"]
    #[inline(always)]
    pub fn sai2sel(&self) -> SAI2SEL_R {
        SAI2SEL_R::new(((self.bits >> 22) & 3) as u8)
    }
    #[doc = "Bit 24 - Timers clocks prescalers selection"]
    #[inline(always)]
    pub fn timpre(&self) -> TIMPRE_R {
        TIMPRE_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - DFSDM1 clock source selection"]
    #[inline(always)]
    pub fn dfsdm1sel(&self) -> DFSDM1SEL_R {
        DFSDM1SEL_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - DFSDM1 AUDIO clock source selection"]
    #[inline(always)]
    pub fn adfsdm1sel(&self) -> ADFSDM1SEL_R {
        ADFSDM1SEL_R::new(((self.bits >> 26) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:4 - PLLI2S division factor for SAI1 clock"]
    #[inline(always)]
    #[must_use]
    pub fn plli2sdivq(&mut self) -> PLLI2SDIVQ_W<0> {
        PLLI2SDIVQ_W::new(self)
    }
    #[doc = "Bits 8:12 - PLLSAI division factor for SAI1 clock"]
    #[inline(always)]
    #[must_use]
    pub fn pllsaidivq(&mut self) -> PLLSAIDIVQ_W<8> {
        PLLSAIDIVQ_W::new(self)
    }
    #[doc = "Bits 16:17 - division factor for LCD_CLK"]
    #[inline(always)]
    #[must_use]
    pub fn pllsaidivr(&mut self) -> PLLSAIDIVR_W<16> {
        PLLSAIDIVR_W::new(self)
    }
    #[doc = "Bits 20:21 - SAI1 clock source selection"]
    #[inline(always)]
    #[must_use]
    pub fn sai1sel(&mut self) -> SAI1SEL_W<20> {
        SAI1SEL_W::new(self)
    }
    #[doc = "Bits 22:23 - SAI2 clock source selection"]
    #[inline(always)]
    #[must_use]
    pub fn sai2sel(&mut self) -> SAI2SEL_W<22> {
        SAI2SEL_W::new(self)
    }
    #[doc = "Bit 24 - Timers clocks prescalers selection"]
    #[inline(always)]
    #[must_use]
    pub fn timpre(&mut self) -> TIMPRE_W<24> {
        TIMPRE_W::new(self)
    }
    #[doc = "Bit 25 - DFSDM1 clock source selection"]
    #[inline(always)]
    #[must_use]
    pub fn dfsdm1sel(&mut self) -> DFSDM1SEL_W<25> {
        DFSDM1SEL_W::new(self)
    }
    #[doc = "Bit 26 - DFSDM1 AUDIO clock source selection"]
    #[inline(always)]
    #[must_use]
    pub fn adfsdm1sel(&mut self) -> ADFSDM1SEL_W<26> {
        ADFSDM1SEL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "dedicated clocks configuration register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dckcfgr1](index.html) module"]
pub struct DCKCFGR1_SPEC;
impl crate::RegisterSpec for DCKCFGR1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dckcfgr1::R](R) reader structure"]
impl crate::Readable for DCKCFGR1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dckcfgr1::W](W) writer structure"]
impl crate::Writable for DCKCFGR1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DCKCFGR1 to value 0"]
impl crate::Resettable for DCKCFGR1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
