#[doc = "Register `PLLCFGR` reader"]
pub struct R(crate::R<PLLCFGR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PLLCFGR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PLLCFGR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PLLCFGR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PLLCFGR` writer"]
pub struct W(crate::W<PLLCFGR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PLLCFGR_SPEC>;
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
impl From<crate::W<PLLCFGR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PLLCFGR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PLLSRC` reader - Main PLL, PLLSAI1 and PLLSAI2 entry clock source"]
pub type PLLSRC_R = crate::FieldReader<u8, PLLSRC_A>;
#[doc = "Main PLL, PLLSAI1 and PLLSAI2 entry clock source\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PLLSRC_A {
    #[doc = "0: No clock sent to PLL"]
    None = 0,
    #[doc = "2: HSI16 sent to PLL input"]
    Hsi16 = 2,
    #[doc = "3: HSE sent to PLL input"]
    Hse = 3,
}
impl From<PLLSRC_A> for u8 {
    #[inline(always)]
    fn from(variant: PLLSRC_A) -> Self {
        variant as _
    }
}
impl PLLSRC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<PLLSRC_A> {
        match self.bits {
            0 => Some(PLLSRC_A::None),
            2 => Some(PLLSRC_A::Hsi16),
            3 => Some(PLLSRC_A::Hse),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `None`"]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == PLLSRC_A::None
    }
    #[doc = "Checks if the value of the field is `Hsi16`"]
    #[inline(always)]
    pub fn is_hsi16(&self) -> bool {
        *self == PLLSRC_A::Hsi16
    }
    #[doc = "Checks if the value of the field is `Hse`"]
    #[inline(always)]
    pub fn is_hse(&self) -> bool {
        *self == PLLSRC_A::Hse
    }
}
#[doc = "Field `PLLSRC` writer - Main PLL, PLLSAI1 and PLLSAI2 entry clock source"]
pub type PLLSRC_W<'a, const O: u8> = crate::FieldWriter<'a, u32, PLLCFGR_SPEC, u8, PLLSRC_A, 2, O>;
impl<'a, const O: u8> PLLSRC_W<'a, O> {
    #[doc = "No clock sent to PLL"]
    #[inline(always)]
    pub fn none(self) -> &'a mut W {
        self.variant(PLLSRC_A::None)
    }
    #[doc = "HSI16 sent to PLL input"]
    #[inline(always)]
    pub fn hsi16(self) -> &'a mut W {
        self.variant(PLLSRC_A::Hsi16)
    }
    #[doc = "HSE sent to PLL input"]
    #[inline(always)]
    pub fn hse(self) -> &'a mut W {
        self.variant(PLLSRC_A::Hse)
    }
}
#[doc = "Field `PLLM` reader - Division factor for the main PLL and audio PLL (PLLSAI1 and PLLSAI2) input clock"]
pub type PLLM_R = crate::FieldReader<u8, PLLM_A>;
#[doc = "Division factor for the main PLL and audio PLL (PLLSAI1 and PLLSAI2) input clock\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PLLM_A {
    #[doc = "0: pll_p_ck = vco_ck / 1"]
    Div1 = 0,
    #[doc = "1: pll_p_ck = vco_ck / 2"]
    Div2 = 1,
    #[doc = "2: pll_p_ck = vco_ck / 3"]
    Div3 = 2,
    #[doc = "3: pll_p_ck = vco_ck / 4"]
    Div4 = 3,
    #[doc = "4: pll_p_ck = vco_ck / 5"]
    Div5 = 4,
    #[doc = "5: pll_p_ck = vco_ck / 6"]
    Div6 = 5,
    #[doc = "6: pll_p_ck = vco_ck / 7"]
    Div7 = 6,
    #[doc = "7: pll_p_ck = vco_ck / 8"]
    Div8 = 7,
    #[doc = "8: pll_p_ck = vco_ck / 9"]
    Div9 = 8,
    #[doc = "9: pll_p_ck = vco_ck / 10"]
    Div10 = 9,
    #[doc = "10: pll_p_ck = vco_ck / 11"]
    Div11 = 10,
    #[doc = "11: pll_p_ck = vco_ck / 12"]
    Div12 = 11,
    #[doc = "12: pll_p_ck = vco_ck / 13"]
    Div13 = 12,
    #[doc = "13: pll_p_ck = vco_ck / 14"]
    Div14 = 13,
    #[doc = "14: pll_p_ck = vco_ck / 15"]
    Div15 = 14,
    #[doc = "15: pll_p_ck = vco_ck / 16"]
    Div16 = 15,
}
impl From<PLLM_A> for u8 {
    #[inline(always)]
    fn from(variant: PLLM_A) -> Self {
        variant as _
    }
}
impl PLLM_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PLLM_A {
        match self.bits {
            0 => PLLM_A::Div1,
            1 => PLLM_A::Div2,
            2 => PLLM_A::Div3,
            3 => PLLM_A::Div4,
            4 => PLLM_A::Div5,
            5 => PLLM_A::Div6,
            6 => PLLM_A::Div7,
            7 => PLLM_A::Div8,
            8 => PLLM_A::Div9,
            9 => PLLM_A::Div10,
            10 => PLLM_A::Div11,
            11 => PLLM_A::Div12,
            12 => PLLM_A::Div13,
            13 => PLLM_A::Div14,
            14 => PLLM_A::Div15,
            15 => PLLM_A::Div16,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `Div1`"]
    #[inline(always)]
    pub fn is_div1(&self) -> bool {
        *self == PLLM_A::Div1
    }
    #[doc = "Checks if the value of the field is `Div2`"]
    #[inline(always)]
    pub fn is_div2(&self) -> bool {
        *self == PLLM_A::Div2
    }
    #[doc = "Checks if the value of the field is `Div3`"]
    #[inline(always)]
    pub fn is_div3(&self) -> bool {
        *self == PLLM_A::Div3
    }
    #[doc = "Checks if the value of the field is `Div4`"]
    #[inline(always)]
    pub fn is_div4(&self) -> bool {
        *self == PLLM_A::Div4
    }
    #[doc = "Checks if the value of the field is `Div5`"]
    #[inline(always)]
    pub fn is_div5(&self) -> bool {
        *self == PLLM_A::Div5
    }
    #[doc = "Checks if the value of the field is `Div6`"]
    #[inline(always)]
    pub fn is_div6(&self) -> bool {
        *self == PLLM_A::Div6
    }
    #[doc = "Checks if the value of the field is `Div7`"]
    #[inline(always)]
    pub fn is_div7(&self) -> bool {
        *self == PLLM_A::Div7
    }
    #[doc = "Checks if the value of the field is `Div8`"]
    #[inline(always)]
    pub fn is_div8(&self) -> bool {
        *self == PLLM_A::Div8
    }
    #[doc = "Checks if the value of the field is `Div9`"]
    #[inline(always)]
    pub fn is_div9(&self) -> bool {
        *self == PLLM_A::Div9
    }
    #[doc = "Checks if the value of the field is `Div10`"]
    #[inline(always)]
    pub fn is_div10(&self) -> bool {
        *self == PLLM_A::Div10
    }
    #[doc = "Checks if the value of the field is `Div11`"]
    #[inline(always)]
    pub fn is_div11(&self) -> bool {
        *self == PLLM_A::Div11
    }
    #[doc = "Checks if the value of the field is `Div12`"]
    #[inline(always)]
    pub fn is_div12(&self) -> bool {
        *self == PLLM_A::Div12
    }
    #[doc = "Checks if the value of the field is `Div13`"]
    #[inline(always)]
    pub fn is_div13(&self) -> bool {
        *self == PLLM_A::Div13
    }
    #[doc = "Checks if the value of the field is `Div14`"]
    #[inline(always)]
    pub fn is_div14(&self) -> bool {
        *self == PLLM_A::Div14
    }
    #[doc = "Checks if the value of the field is `Div15`"]
    #[inline(always)]
    pub fn is_div15(&self) -> bool {
        *self == PLLM_A::Div15
    }
    #[doc = "Checks if the value of the field is `Div16`"]
    #[inline(always)]
    pub fn is_div16(&self) -> bool {
        *self == PLLM_A::Div16
    }
}
#[doc = "Field `PLLM` writer - Division factor for the main PLL and audio PLL (PLLSAI1 and PLLSAI2) input clock"]
pub type PLLM_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, PLLCFGR_SPEC, u8, PLLM_A, 4, O>;
impl<'a, const O: u8> PLLM_W<'a, O> {
    #[doc = "pll_p_ck = vco_ck / 1"]
    #[inline(always)]
    pub fn div1(self) -> &'a mut W {
        self.variant(PLLM_A::Div1)
    }
    #[doc = "pll_p_ck = vco_ck / 2"]
    #[inline(always)]
    pub fn div2(self) -> &'a mut W {
        self.variant(PLLM_A::Div2)
    }
    #[doc = "pll_p_ck = vco_ck / 3"]
    #[inline(always)]
    pub fn div3(self) -> &'a mut W {
        self.variant(PLLM_A::Div3)
    }
    #[doc = "pll_p_ck = vco_ck / 4"]
    #[inline(always)]
    pub fn div4(self) -> &'a mut W {
        self.variant(PLLM_A::Div4)
    }
    #[doc = "pll_p_ck = vco_ck / 5"]
    #[inline(always)]
    pub fn div5(self) -> &'a mut W {
        self.variant(PLLM_A::Div5)
    }
    #[doc = "pll_p_ck = vco_ck / 6"]
    #[inline(always)]
    pub fn div6(self) -> &'a mut W {
        self.variant(PLLM_A::Div6)
    }
    #[doc = "pll_p_ck = vco_ck / 7"]
    #[inline(always)]
    pub fn div7(self) -> &'a mut W {
        self.variant(PLLM_A::Div7)
    }
    #[doc = "pll_p_ck = vco_ck / 8"]
    #[inline(always)]
    pub fn div8(self) -> &'a mut W {
        self.variant(PLLM_A::Div8)
    }
    #[doc = "pll_p_ck = vco_ck / 9"]
    #[inline(always)]
    pub fn div9(self) -> &'a mut W {
        self.variant(PLLM_A::Div9)
    }
    #[doc = "pll_p_ck = vco_ck / 10"]
    #[inline(always)]
    pub fn div10(self) -> &'a mut W {
        self.variant(PLLM_A::Div10)
    }
    #[doc = "pll_p_ck = vco_ck / 11"]
    #[inline(always)]
    pub fn div11(self) -> &'a mut W {
        self.variant(PLLM_A::Div11)
    }
    #[doc = "pll_p_ck = vco_ck / 12"]
    #[inline(always)]
    pub fn div12(self) -> &'a mut W {
        self.variant(PLLM_A::Div12)
    }
    #[doc = "pll_p_ck = vco_ck / 13"]
    #[inline(always)]
    pub fn div13(self) -> &'a mut W {
        self.variant(PLLM_A::Div13)
    }
    #[doc = "pll_p_ck = vco_ck / 14"]
    #[inline(always)]
    pub fn div14(self) -> &'a mut W {
        self.variant(PLLM_A::Div14)
    }
    #[doc = "pll_p_ck = vco_ck / 15"]
    #[inline(always)]
    pub fn div15(self) -> &'a mut W {
        self.variant(PLLM_A::Div15)
    }
    #[doc = "pll_p_ck = vco_ck / 16"]
    #[inline(always)]
    pub fn div16(self) -> &'a mut W {
        self.variant(PLLM_A::Div16)
    }
}
#[doc = "Field `PLLN` reader - Main PLL multiplication factor for VCO"]
pub type PLLN_R = crate::FieldReader<u8, PLLN_A>;
#[doc = "Main PLL multiplication factor for VCO\n\nValue on reset: 16"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PLLN_A {
    #[doc = "8: pll_n_ck = vco_ck / 8"]
    Div8 = 8,
    #[doc = "9: pll_n_ck = vco_ck / 9"]
    Div9 = 9,
    #[doc = "10: pll_n_ck = vco_ck / 10"]
    Div10 = 10,
    #[doc = "11: pll_n_ck = vco_ck / 11"]
    Div11 = 11,
    #[doc = "12: pll_n_ck = vco_ck / 12"]
    Div12 = 12,
    #[doc = "13: pll_n_ck = vco_ck / 13"]
    Div13 = 13,
    #[doc = "14: pll_n_ck = vco_ck / 14"]
    Div14 = 14,
    #[doc = "15: pll_n_ck = vco_ck / 15"]
    Div15 = 15,
    #[doc = "16: pll_n_ck = vco_ck / 16"]
    Div16 = 16,
    #[doc = "17: pll_n_ck = vco_ck / 17"]
    Div17 = 17,
    #[doc = "18: pll_n_ck = vco_ck / 18"]
    Div18 = 18,
    #[doc = "19: pll_n_ck = vco_ck / 19"]
    Div19 = 19,
    #[doc = "20: pll_n_ck = vco_ck / 20"]
    Div20 = 20,
    #[doc = "21: pll_n_ck = vco_ck / 21"]
    Div21 = 21,
    #[doc = "22: pll_n_ck = vco_ck / 22"]
    Div22 = 22,
    #[doc = "23: pll_n_ck = vco_ck / 23"]
    Div23 = 23,
    #[doc = "24: pll_n_ck = vco_ck / 24"]
    Div24 = 24,
    #[doc = "25: pll_n_ck = vco_ck / 25"]
    Div25 = 25,
    #[doc = "26: pll_n_ck = vco_ck / 26"]
    Div26 = 26,
    #[doc = "27: pll_n_ck = vco_ck / 27"]
    Div27 = 27,
    #[doc = "28: pll_n_ck = vco_ck / 28"]
    Div28 = 28,
    #[doc = "29: pll_n_ck = vco_ck / 29"]
    Div29 = 29,
    #[doc = "30: pll_n_ck = vco_ck / 30"]
    Div30 = 30,
    #[doc = "31: pll_n_ck = vco_ck / 31"]
    Div31 = 31,
    #[doc = "32: pll_n_ck = vco_ck / 32"]
    Div32 = 32,
    #[doc = "33: pll_n_ck = vco_ck / 33"]
    Div33 = 33,
    #[doc = "34: pll_n_ck = vco_ck / 34"]
    Div34 = 34,
    #[doc = "35: pll_n_ck = vco_ck / 35"]
    Div35 = 35,
    #[doc = "36: pll_n_ck = vco_ck / 36"]
    Div36 = 36,
    #[doc = "37: pll_n_ck = vco_ck / 37"]
    Div37 = 37,
    #[doc = "38: pll_n_ck = vco_ck / 38"]
    Div38 = 38,
    #[doc = "39: pll_n_ck = vco_ck / 39"]
    Div39 = 39,
    #[doc = "40: pll_n_ck = vco_ck / 40"]
    Div40 = 40,
    #[doc = "41: pll_n_ck = vco_ck / 41"]
    Div41 = 41,
    #[doc = "42: pll_n_ck = vco_ck / 42"]
    Div42 = 42,
    #[doc = "43: pll_n_ck = vco_ck / 43"]
    Div43 = 43,
    #[doc = "44: pll_n_ck = vco_ck / 44"]
    Div44 = 44,
    #[doc = "45: pll_n_ck = vco_ck / 45"]
    Div45 = 45,
    #[doc = "46: pll_n_ck = vco_ck / 46"]
    Div46 = 46,
    #[doc = "47: pll_n_ck = vco_ck / 47"]
    Div47 = 47,
    #[doc = "48: pll_n_ck = vco_ck / 48"]
    Div48 = 48,
    #[doc = "49: pll_n_ck = vco_ck / 49"]
    Div49 = 49,
    #[doc = "50: pll_n_ck = vco_ck / 50"]
    Div50 = 50,
    #[doc = "51: pll_n_ck = vco_ck / 51"]
    Div51 = 51,
    #[doc = "52: pll_n_ck = vco_ck / 52"]
    Div52 = 52,
    #[doc = "53: pll_n_ck = vco_ck / 53"]
    Div53 = 53,
    #[doc = "54: pll_n_ck = vco_ck / 54"]
    Div54 = 54,
    #[doc = "55: pll_n_ck = vco_ck / 55"]
    Div55 = 55,
    #[doc = "56: pll_n_ck = vco_ck / 56"]
    Div56 = 56,
    #[doc = "57: pll_n_ck = vco_ck / 57"]
    Div57 = 57,
    #[doc = "58: pll_n_ck = vco_ck / 58"]
    Div58 = 58,
    #[doc = "59: pll_n_ck = vco_ck / 59"]
    Div59 = 59,
    #[doc = "60: pll_n_ck = vco_ck / 60"]
    Div60 = 60,
    #[doc = "61: pll_n_ck = vco_ck / 61"]
    Div61 = 61,
    #[doc = "62: pll_n_ck = vco_ck / 62"]
    Div62 = 62,
    #[doc = "63: pll_n_ck = vco_ck / 63"]
    Div63 = 63,
    #[doc = "64: pll_n_ck = vco_ck / 64"]
    Div64 = 64,
    #[doc = "65: pll_n_ck = vco_ck / 65"]
    Div65 = 65,
    #[doc = "66: pll_n_ck = vco_ck / 66"]
    Div66 = 66,
    #[doc = "67: pll_n_ck = vco_ck / 67"]
    Div67 = 67,
    #[doc = "68: pll_n_ck = vco_ck / 68"]
    Div68 = 68,
    #[doc = "69: pll_n_ck = vco_ck / 69"]
    Div69 = 69,
    #[doc = "70: pll_n_ck = vco_ck / 70"]
    Div70 = 70,
    #[doc = "71: pll_n_ck = vco_ck / 71"]
    Div71 = 71,
    #[doc = "72: pll_n_ck = vco_ck / 72"]
    Div72 = 72,
    #[doc = "73: pll_n_ck = vco_ck / 73"]
    Div73 = 73,
    #[doc = "74: pll_n_ck = vco_ck / 74"]
    Div74 = 74,
    #[doc = "75: pll_n_ck = vco_ck / 75"]
    Div75 = 75,
    #[doc = "76: pll_n_ck = vco_ck / 76"]
    Div76 = 76,
    #[doc = "77: pll_n_ck = vco_ck / 77"]
    Div77 = 77,
    #[doc = "78: pll_n_ck = vco_ck / 78"]
    Div78 = 78,
    #[doc = "79: pll_n_ck = vco_ck / 79"]
    Div79 = 79,
    #[doc = "80: pll_n_ck = vco_ck / 80"]
    Div80 = 80,
    #[doc = "81: pll_n_ck = vco_ck / 81"]
    Div81 = 81,
    #[doc = "82: pll_n_ck = vco_ck / 82"]
    Div82 = 82,
    #[doc = "83: pll_n_ck = vco_ck / 83"]
    Div83 = 83,
    #[doc = "84: pll_n_ck = vco_ck / 84"]
    Div84 = 84,
    #[doc = "85: pll_n_ck = vco_ck / 85"]
    Div85 = 85,
    #[doc = "86: pll_n_ck = vco_ck / 86"]
    Div86 = 86,
    #[doc = "87: pll_n_ck = vco_ck / 87"]
    Div87 = 87,
    #[doc = "88: pll_n_ck = vco_ck / 88"]
    Div88 = 88,
    #[doc = "89: pll_n_ck = vco_ck / 89"]
    Div89 = 89,
    #[doc = "90: pll_n_ck = vco_ck / 90"]
    Div90 = 90,
    #[doc = "91: pll_n_ck = vco_ck / 91"]
    Div91 = 91,
    #[doc = "92: pll_n_ck = vco_ck / 92"]
    Div92 = 92,
    #[doc = "93: pll_n_ck = vco_ck / 93"]
    Div93 = 93,
    #[doc = "94: pll_n_ck = vco_ck / 94"]
    Div94 = 94,
    #[doc = "95: pll_n_ck = vco_ck / 95"]
    Div95 = 95,
    #[doc = "96: pll_n_ck = vco_ck / 96"]
    Div96 = 96,
    #[doc = "97: pll_n_ck = vco_ck / 97"]
    Div97 = 97,
    #[doc = "98: pll_n_ck = vco_ck / 98"]
    Div98 = 98,
    #[doc = "99: pll_n_ck = vco_ck / 99"]
    Div99 = 99,
    #[doc = "100: pll_n_ck = vco_ck / 100"]
    Div100 = 100,
    #[doc = "101: pll_n_ck = vco_ck / 101"]
    Div101 = 101,
    #[doc = "102: pll_n_ck = vco_ck / 102"]
    Div102 = 102,
    #[doc = "103: pll_n_ck = vco_ck / 103"]
    Div103 = 103,
    #[doc = "104: pll_n_ck = vco_ck / 104"]
    Div104 = 104,
    #[doc = "105: pll_n_ck = vco_ck / 105"]
    Div105 = 105,
    #[doc = "106: pll_n_ck = vco_ck / 106"]
    Div106 = 106,
    #[doc = "107: pll_n_ck = vco_ck / 107"]
    Div107 = 107,
    #[doc = "108: pll_n_ck = vco_ck / 108"]
    Div108 = 108,
    #[doc = "109: pll_n_ck = vco_ck / 109"]
    Div109 = 109,
    #[doc = "110: pll_n_ck = vco_ck / 110"]
    Div110 = 110,
    #[doc = "111: pll_n_ck = vco_ck / 111"]
    Div111 = 111,
    #[doc = "112: pll_n_ck = vco_ck / 112"]
    Div112 = 112,
    #[doc = "113: pll_n_ck = vco_ck / 113"]
    Div113 = 113,
    #[doc = "114: pll_n_ck = vco_ck / 114"]
    Div114 = 114,
    #[doc = "115: pll_n_ck = vco_ck / 115"]
    Div115 = 115,
    #[doc = "116: pll_n_ck = vco_ck / 116"]
    Div116 = 116,
    #[doc = "117: pll_n_ck = vco_ck / 117"]
    Div117 = 117,
    #[doc = "118: pll_n_ck = vco_ck / 118"]
    Div118 = 118,
    #[doc = "119: pll_n_ck = vco_ck / 119"]
    Div119 = 119,
    #[doc = "120: pll_n_ck = vco_ck / 120"]
    Div120 = 120,
    #[doc = "121: pll_n_ck = vco_ck / 121"]
    Div121 = 121,
    #[doc = "122: pll_n_ck = vco_ck / 122"]
    Div122 = 122,
    #[doc = "123: pll_n_ck = vco_ck / 123"]
    Div123 = 123,
    #[doc = "124: pll_n_ck = vco_ck / 124"]
    Div124 = 124,
    #[doc = "125: pll_n_ck = vco_ck / 125"]
    Div125 = 125,
    #[doc = "126: pll_n_ck = vco_ck / 126"]
    Div126 = 126,
    #[doc = "127: pll_n_ck = vco_ck / 127"]
    Div127 = 127,
}
impl From<PLLN_A> for u8 {
    #[inline(always)]
    fn from(variant: PLLN_A) -> Self {
        variant as _
    }
}
impl PLLN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<PLLN_A> {
        match self.bits {
            8 => Some(PLLN_A::Div8),
            9 => Some(PLLN_A::Div9),
            10 => Some(PLLN_A::Div10),
            11 => Some(PLLN_A::Div11),
            12 => Some(PLLN_A::Div12),
            13 => Some(PLLN_A::Div13),
            14 => Some(PLLN_A::Div14),
            15 => Some(PLLN_A::Div15),
            16 => Some(PLLN_A::Div16),
            17 => Some(PLLN_A::Div17),
            18 => Some(PLLN_A::Div18),
            19 => Some(PLLN_A::Div19),
            20 => Some(PLLN_A::Div20),
            21 => Some(PLLN_A::Div21),
            22 => Some(PLLN_A::Div22),
            23 => Some(PLLN_A::Div23),
            24 => Some(PLLN_A::Div24),
            25 => Some(PLLN_A::Div25),
            26 => Some(PLLN_A::Div26),
            27 => Some(PLLN_A::Div27),
            28 => Some(PLLN_A::Div28),
            29 => Some(PLLN_A::Div29),
            30 => Some(PLLN_A::Div30),
            31 => Some(PLLN_A::Div31),
            32 => Some(PLLN_A::Div32),
            33 => Some(PLLN_A::Div33),
            34 => Some(PLLN_A::Div34),
            35 => Some(PLLN_A::Div35),
            36 => Some(PLLN_A::Div36),
            37 => Some(PLLN_A::Div37),
            38 => Some(PLLN_A::Div38),
            39 => Some(PLLN_A::Div39),
            40 => Some(PLLN_A::Div40),
            41 => Some(PLLN_A::Div41),
            42 => Some(PLLN_A::Div42),
            43 => Some(PLLN_A::Div43),
            44 => Some(PLLN_A::Div44),
            45 => Some(PLLN_A::Div45),
            46 => Some(PLLN_A::Div46),
            47 => Some(PLLN_A::Div47),
            48 => Some(PLLN_A::Div48),
            49 => Some(PLLN_A::Div49),
            50 => Some(PLLN_A::Div50),
            51 => Some(PLLN_A::Div51),
            52 => Some(PLLN_A::Div52),
            53 => Some(PLLN_A::Div53),
            54 => Some(PLLN_A::Div54),
            55 => Some(PLLN_A::Div55),
            56 => Some(PLLN_A::Div56),
            57 => Some(PLLN_A::Div57),
            58 => Some(PLLN_A::Div58),
            59 => Some(PLLN_A::Div59),
            60 => Some(PLLN_A::Div60),
            61 => Some(PLLN_A::Div61),
            62 => Some(PLLN_A::Div62),
            63 => Some(PLLN_A::Div63),
            64 => Some(PLLN_A::Div64),
            65 => Some(PLLN_A::Div65),
            66 => Some(PLLN_A::Div66),
            67 => Some(PLLN_A::Div67),
            68 => Some(PLLN_A::Div68),
            69 => Some(PLLN_A::Div69),
            70 => Some(PLLN_A::Div70),
            71 => Some(PLLN_A::Div71),
            72 => Some(PLLN_A::Div72),
            73 => Some(PLLN_A::Div73),
            74 => Some(PLLN_A::Div74),
            75 => Some(PLLN_A::Div75),
            76 => Some(PLLN_A::Div76),
            77 => Some(PLLN_A::Div77),
            78 => Some(PLLN_A::Div78),
            79 => Some(PLLN_A::Div79),
            80 => Some(PLLN_A::Div80),
            81 => Some(PLLN_A::Div81),
            82 => Some(PLLN_A::Div82),
            83 => Some(PLLN_A::Div83),
            84 => Some(PLLN_A::Div84),
            85 => Some(PLLN_A::Div85),
            86 => Some(PLLN_A::Div86),
            87 => Some(PLLN_A::Div87),
            88 => Some(PLLN_A::Div88),
            89 => Some(PLLN_A::Div89),
            90 => Some(PLLN_A::Div90),
            91 => Some(PLLN_A::Div91),
            92 => Some(PLLN_A::Div92),
            93 => Some(PLLN_A::Div93),
            94 => Some(PLLN_A::Div94),
            95 => Some(PLLN_A::Div95),
            96 => Some(PLLN_A::Div96),
            97 => Some(PLLN_A::Div97),
            98 => Some(PLLN_A::Div98),
            99 => Some(PLLN_A::Div99),
            100 => Some(PLLN_A::Div100),
            101 => Some(PLLN_A::Div101),
            102 => Some(PLLN_A::Div102),
            103 => Some(PLLN_A::Div103),
            104 => Some(PLLN_A::Div104),
            105 => Some(PLLN_A::Div105),
            106 => Some(PLLN_A::Div106),
            107 => Some(PLLN_A::Div107),
            108 => Some(PLLN_A::Div108),
            109 => Some(PLLN_A::Div109),
            110 => Some(PLLN_A::Div110),
            111 => Some(PLLN_A::Div111),
            112 => Some(PLLN_A::Div112),
            113 => Some(PLLN_A::Div113),
            114 => Some(PLLN_A::Div114),
            115 => Some(PLLN_A::Div115),
            116 => Some(PLLN_A::Div116),
            117 => Some(PLLN_A::Div117),
            118 => Some(PLLN_A::Div118),
            119 => Some(PLLN_A::Div119),
            120 => Some(PLLN_A::Div120),
            121 => Some(PLLN_A::Div121),
            122 => Some(PLLN_A::Div122),
            123 => Some(PLLN_A::Div123),
            124 => Some(PLLN_A::Div124),
            125 => Some(PLLN_A::Div125),
            126 => Some(PLLN_A::Div126),
            127 => Some(PLLN_A::Div127),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `Div8`"]
    #[inline(always)]
    pub fn is_div8(&self) -> bool {
        *self == PLLN_A::Div8
    }
    #[doc = "Checks if the value of the field is `Div9`"]
    #[inline(always)]
    pub fn is_div9(&self) -> bool {
        *self == PLLN_A::Div9
    }
    #[doc = "Checks if the value of the field is `Div10`"]
    #[inline(always)]
    pub fn is_div10(&self) -> bool {
        *self == PLLN_A::Div10
    }
    #[doc = "Checks if the value of the field is `Div11`"]
    #[inline(always)]
    pub fn is_div11(&self) -> bool {
        *self == PLLN_A::Div11
    }
    #[doc = "Checks if the value of the field is `Div12`"]
    #[inline(always)]
    pub fn is_div12(&self) -> bool {
        *self == PLLN_A::Div12
    }
    #[doc = "Checks if the value of the field is `Div13`"]
    #[inline(always)]
    pub fn is_div13(&self) -> bool {
        *self == PLLN_A::Div13
    }
    #[doc = "Checks if the value of the field is `Div14`"]
    #[inline(always)]
    pub fn is_div14(&self) -> bool {
        *self == PLLN_A::Div14
    }
    #[doc = "Checks if the value of the field is `Div15`"]
    #[inline(always)]
    pub fn is_div15(&self) -> bool {
        *self == PLLN_A::Div15
    }
    #[doc = "Checks if the value of the field is `Div16`"]
    #[inline(always)]
    pub fn is_div16(&self) -> bool {
        *self == PLLN_A::Div16
    }
    #[doc = "Checks if the value of the field is `Div17`"]
    #[inline(always)]
    pub fn is_div17(&self) -> bool {
        *self == PLLN_A::Div17
    }
    #[doc = "Checks if the value of the field is `Div18`"]
    #[inline(always)]
    pub fn is_div18(&self) -> bool {
        *self == PLLN_A::Div18
    }
    #[doc = "Checks if the value of the field is `Div19`"]
    #[inline(always)]
    pub fn is_div19(&self) -> bool {
        *self == PLLN_A::Div19
    }
    #[doc = "Checks if the value of the field is `Div20`"]
    #[inline(always)]
    pub fn is_div20(&self) -> bool {
        *self == PLLN_A::Div20
    }
    #[doc = "Checks if the value of the field is `Div21`"]
    #[inline(always)]
    pub fn is_div21(&self) -> bool {
        *self == PLLN_A::Div21
    }
    #[doc = "Checks if the value of the field is `Div22`"]
    #[inline(always)]
    pub fn is_div22(&self) -> bool {
        *self == PLLN_A::Div22
    }
    #[doc = "Checks if the value of the field is `Div23`"]
    #[inline(always)]
    pub fn is_div23(&self) -> bool {
        *self == PLLN_A::Div23
    }
    #[doc = "Checks if the value of the field is `Div24`"]
    #[inline(always)]
    pub fn is_div24(&self) -> bool {
        *self == PLLN_A::Div24
    }
    #[doc = "Checks if the value of the field is `Div25`"]
    #[inline(always)]
    pub fn is_div25(&self) -> bool {
        *self == PLLN_A::Div25
    }
    #[doc = "Checks if the value of the field is `Div26`"]
    #[inline(always)]
    pub fn is_div26(&self) -> bool {
        *self == PLLN_A::Div26
    }
    #[doc = "Checks if the value of the field is `Div27`"]
    #[inline(always)]
    pub fn is_div27(&self) -> bool {
        *self == PLLN_A::Div27
    }
    #[doc = "Checks if the value of the field is `Div28`"]
    #[inline(always)]
    pub fn is_div28(&self) -> bool {
        *self == PLLN_A::Div28
    }
    #[doc = "Checks if the value of the field is `Div29`"]
    #[inline(always)]
    pub fn is_div29(&self) -> bool {
        *self == PLLN_A::Div29
    }
    #[doc = "Checks if the value of the field is `Div30`"]
    #[inline(always)]
    pub fn is_div30(&self) -> bool {
        *self == PLLN_A::Div30
    }
    #[doc = "Checks if the value of the field is `Div31`"]
    #[inline(always)]
    pub fn is_div31(&self) -> bool {
        *self == PLLN_A::Div31
    }
    #[doc = "Checks if the value of the field is `Div32`"]
    #[inline(always)]
    pub fn is_div32(&self) -> bool {
        *self == PLLN_A::Div32
    }
    #[doc = "Checks if the value of the field is `Div33`"]
    #[inline(always)]
    pub fn is_div33(&self) -> bool {
        *self == PLLN_A::Div33
    }
    #[doc = "Checks if the value of the field is `Div34`"]
    #[inline(always)]
    pub fn is_div34(&self) -> bool {
        *self == PLLN_A::Div34
    }
    #[doc = "Checks if the value of the field is `Div35`"]
    #[inline(always)]
    pub fn is_div35(&self) -> bool {
        *self == PLLN_A::Div35
    }
    #[doc = "Checks if the value of the field is `Div36`"]
    #[inline(always)]
    pub fn is_div36(&self) -> bool {
        *self == PLLN_A::Div36
    }
    #[doc = "Checks if the value of the field is `Div37`"]
    #[inline(always)]
    pub fn is_div37(&self) -> bool {
        *self == PLLN_A::Div37
    }
    #[doc = "Checks if the value of the field is `Div38`"]
    #[inline(always)]
    pub fn is_div38(&self) -> bool {
        *self == PLLN_A::Div38
    }
    #[doc = "Checks if the value of the field is `Div39`"]
    #[inline(always)]
    pub fn is_div39(&self) -> bool {
        *self == PLLN_A::Div39
    }
    #[doc = "Checks if the value of the field is `Div40`"]
    #[inline(always)]
    pub fn is_div40(&self) -> bool {
        *self == PLLN_A::Div40
    }
    #[doc = "Checks if the value of the field is `Div41`"]
    #[inline(always)]
    pub fn is_div41(&self) -> bool {
        *self == PLLN_A::Div41
    }
    #[doc = "Checks if the value of the field is `Div42`"]
    #[inline(always)]
    pub fn is_div42(&self) -> bool {
        *self == PLLN_A::Div42
    }
    #[doc = "Checks if the value of the field is `Div43`"]
    #[inline(always)]
    pub fn is_div43(&self) -> bool {
        *self == PLLN_A::Div43
    }
    #[doc = "Checks if the value of the field is `Div44`"]
    #[inline(always)]
    pub fn is_div44(&self) -> bool {
        *self == PLLN_A::Div44
    }
    #[doc = "Checks if the value of the field is `Div45`"]
    #[inline(always)]
    pub fn is_div45(&self) -> bool {
        *self == PLLN_A::Div45
    }
    #[doc = "Checks if the value of the field is `Div46`"]
    #[inline(always)]
    pub fn is_div46(&self) -> bool {
        *self == PLLN_A::Div46
    }
    #[doc = "Checks if the value of the field is `Div47`"]
    #[inline(always)]
    pub fn is_div47(&self) -> bool {
        *self == PLLN_A::Div47
    }
    #[doc = "Checks if the value of the field is `Div48`"]
    #[inline(always)]
    pub fn is_div48(&self) -> bool {
        *self == PLLN_A::Div48
    }
    #[doc = "Checks if the value of the field is `Div49`"]
    #[inline(always)]
    pub fn is_div49(&self) -> bool {
        *self == PLLN_A::Div49
    }
    #[doc = "Checks if the value of the field is `Div50`"]
    #[inline(always)]
    pub fn is_div50(&self) -> bool {
        *self == PLLN_A::Div50
    }
    #[doc = "Checks if the value of the field is `Div51`"]
    #[inline(always)]
    pub fn is_div51(&self) -> bool {
        *self == PLLN_A::Div51
    }
    #[doc = "Checks if the value of the field is `Div52`"]
    #[inline(always)]
    pub fn is_div52(&self) -> bool {
        *self == PLLN_A::Div52
    }
    #[doc = "Checks if the value of the field is `Div53`"]
    #[inline(always)]
    pub fn is_div53(&self) -> bool {
        *self == PLLN_A::Div53
    }
    #[doc = "Checks if the value of the field is `Div54`"]
    #[inline(always)]
    pub fn is_div54(&self) -> bool {
        *self == PLLN_A::Div54
    }
    #[doc = "Checks if the value of the field is `Div55`"]
    #[inline(always)]
    pub fn is_div55(&self) -> bool {
        *self == PLLN_A::Div55
    }
    #[doc = "Checks if the value of the field is `Div56`"]
    #[inline(always)]
    pub fn is_div56(&self) -> bool {
        *self == PLLN_A::Div56
    }
    #[doc = "Checks if the value of the field is `Div57`"]
    #[inline(always)]
    pub fn is_div57(&self) -> bool {
        *self == PLLN_A::Div57
    }
    #[doc = "Checks if the value of the field is `Div58`"]
    #[inline(always)]
    pub fn is_div58(&self) -> bool {
        *self == PLLN_A::Div58
    }
    #[doc = "Checks if the value of the field is `Div59`"]
    #[inline(always)]
    pub fn is_div59(&self) -> bool {
        *self == PLLN_A::Div59
    }
    #[doc = "Checks if the value of the field is `Div60`"]
    #[inline(always)]
    pub fn is_div60(&self) -> bool {
        *self == PLLN_A::Div60
    }
    #[doc = "Checks if the value of the field is `Div61`"]
    #[inline(always)]
    pub fn is_div61(&self) -> bool {
        *self == PLLN_A::Div61
    }
    #[doc = "Checks if the value of the field is `Div62`"]
    #[inline(always)]
    pub fn is_div62(&self) -> bool {
        *self == PLLN_A::Div62
    }
    #[doc = "Checks if the value of the field is `Div63`"]
    #[inline(always)]
    pub fn is_div63(&self) -> bool {
        *self == PLLN_A::Div63
    }
    #[doc = "Checks if the value of the field is `Div64`"]
    #[inline(always)]
    pub fn is_div64(&self) -> bool {
        *self == PLLN_A::Div64
    }
    #[doc = "Checks if the value of the field is `Div65`"]
    #[inline(always)]
    pub fn is_div65(&self) -> bool {
        *self == PLLN_A::Div65
    }
    #[doc = "Checks if the value of the field is `Div66`"]
    #[inline(always)]
    pub fn is_div66(&self) -> bool {
        *self == PLLN_A::Div66
    }
    #[doc = "Checks if the value of the field is `Div67`"]
    #[inline(always)]
    pub fn is_div67(&self) -> bool {
        *self == PLLN_A::Div67
    }
    #[doc = "Checks if the value of the field is `Div68`"]
    #[inline(always)]
    pub fn is_div68(&self) -> bool {
        *self == PLLN_A::Div68
    }
    #[doc = "Checks if the value of the field is `Div69`"]
    #[inline(always)]
    pub fn is_div69(&self) -> bool {
        *self == PLLN_A::Div69
    }
    #[doc = "Checks if the value of the field is `Div70`"]
    #[inline(always)]
    pub fn is_div70(&self) -> bool {
        *self == PLLN_A::Div70
    }
    #[doc = "Checks if the value of the field is `Div71`"]
    #[inline(always)]
    pub fn is_div71(&self) -> bool {
        *self == PLLN_A::Div71
    }
    #[doc = "Checks if the value of the field is `Div72`"]
    #[inline(always)]
    pub fn is_div72(&self) -> bool {
        *self == PLLN_A::Div72
    }
    #[doc = "Checks if the value of the field is `Div73`"]
    #[inline(always)]
    pub fn is_div73(&self) -> bool {
        *self == PLLN_A::Div73
    }
    #[doc = "Checks if the value of the field is `Div74`"]
    #[inline(always)]
    pub fn is_div74(&self) -> bool {
        *self == PLLN_A::Div74
    }
    #[doc = "Checks if the value of the field is `Div75`"]
    #[inline(always)]
    pub fn is_div75(&self) -> bool {
        *self == PLLN_A::Div75
    }
    #[doc = "Checks if the value of the field is `Div76`"]
    #[inline(always)]
    pub fn is_div76(&self) -> bool {
        *self == PLLN_A::Div76
    }
    #[doc = "Checks if the value of the field is `Div77`"]
    #[inline(always)]
    pub fn is_div77(&self) -> bool {
        *self == PLLN_A::Div77
    }
    #[doc = "Checks if the value of the field is `Div78`"]
    #[inline(always)]
    pub fn is_div78(&self) -> bool {
        *self == PLLN_A::Div78
    }
    #[doc = "Checks if the value of the field is `Div79`"]
    #[inline(always)]
    pub fn is_div79(&self) -> bool {
        *self == PLLN_A::Div79
    }
    #[doc = "Checks if the value of the field is `Div80`"]
    #[inline(always)]
    pub fn is_div80(&self) -> bool {
        *self == PLLN_A::Div80
    }
    #[doc = "Checks if the value of the field is `Div81`"]
    #[inline(always)]
    pub fn is_div81(&self) -> bool {
        *self == PLLN_A::Div81
    }
    #[doc = "Checks if the value of the field is `Div82`"]
    #[inline(always)]
    pub fn is_div82(&self) -> bool {
        *self == PLLN_A::Div82
    }
    #[doc = "Checks if the value of the field is `Div83`"]
    #[inline(always)]
    pub fn is_div83(&self) -> bool {
        *self == PLLN_A::Div83
    }
    #[doc = "Checks if the value of the field is `Div84`"]
    #[inline(always)]
    pub fn is_div84(&self) -> bool {
        *self == PLLN_A::Div84
    }
    #[doc = "Checks if the value of the field is `Div85`"]
    #[inline(always)]
    pub fn is_div85(&self) -> bool {
        *self == PLLN_A::Div85
    }
    #[doc = "Checks if the value of the field is `Div86`"]
    #[inline(always)]
    pub fn is_div86(&self) -> bool {
        *self == PLLN_A::Div86
    }
    #[doc = "Checks if the value of the field is `Div87`"]
    #[inline(always)]
    pub fn is_div87(&self) -> bool {
        *self == PLLN_A::Div87
    }
    #[doc = "Checks if the value of the field is `Div88`"]
    #[inline(always)]
    pub fn is_div88(&self) -> bool {
        *self == PLLN_A::Div88
    }
    #[doc = "Checks if the value of the field is `Div89`"]
    #[inline(always)]
    pub fn is_div89(&self) -> bool {
        *self == PLLN_A::Div89
    }
    #[doc = "Checks if the value of the field is `Div90`"]
    #[inline(always)]
    pub fn is_div90(&self) -> bool {
        *self == PLLN_A::Div90
    }
    #[doc = "Checks if the value of the field is `Div91`"]
    #[inline(always)]
    pub fn is_div91(&self) -> bool {
        *self == PLLN_A::Div91
    }
    #[doc = "Checks if the value of the field is `Div92`"]
    #[inline(always)]
    pub fn is_div92(&self) -> bool {
        *self == PLLN_A::Div92
    }
    #[doc = "Checks if the value of the field is `Div93`"]
    #[inline(always)]
    pub fn is_div93(&self) -> bool {
        *self == PLLN_A::Div93
    }
    #[doc = "Checks if the value of the field is `Div94`"]
    #[inline(always)]
    pub fn is_div94(&self) -> bool {
        *self == PLLN_A::Div94
    }
    #[doc = "Checks if the value of the field is `Div95`"]
    #[inline(always)]
    pub fn is_div95(&self) -> bool {
        *self == PLLN_A::Div95
    }
    #[doc = "Checks if the value of the field is `Div96`"]
    #[inline(always)]
    pub fn is_div96(&self) -> bool {
        *self == PLLN_A::Div96
    }
    #[doc = "Checks if the value of the field is `Div97`"]
    #[inline(always)]
    pub fn is_div97(&self) -> bool {
        *self == PLLN_A::Div97
    }
    #[doc = "Checks if the value of the field is `Div98`"]
    #[inline(always)]
    pub fn is_div98(&self) -> bool {
        *self == PLLN_A::Div98
    }
    #[doc = "Checks if the value of the field is `Div99`"]
    #[inline(always)]
    pub fn is_div99(&self) -> bool {
        *self == PLLN_A::Div99
    }
    #[doc = "Checks if the value of the field is `Div100`"]
    #[inline(always)]
    pub fn is_div100(&self) -> bool {
        *self == PLLN_A::Div100
    }
    #[doc = "Checks if the value of the field is `Div101`"]
    #[inline(always)]
    pub fn is_div101(&self) -> bool {
        *self == PLLN_A::Div101
    }
    #[doc = "Checks if the value of the field is `Div102`"]
    #[inline(always)]
    pub fn is_div102(&self) -> bool {
        *self == PLLN_A::Div102
    }
    #[doc = "Checks if the value of the field is `Div103`"]
    #[inline(always)]
    pub fn is_div103(&self) -> bool {
        *self == PLLN_A::Div103
    }
    #[doc = "Checks if the value of the field is `Div104`"]
    #[inline(always)]
    pub fn is_div104(&self) -> bool {
        *self == PLLN_A::Div104
    }
    #[doc = "Checks if the value of the field is `Div105`"]
    #[inline(always)]
    pub fn is_div105(&self) -> bool {
        *self == PLLN_A::Div105
    }
    #[doc = "Checks if the value of the field is `Div106`"]
    #[inline(always)]
    pub fn is_div106(&self) -> bool {
        *self == PLLN_A::Div106
    }
    #[doc = "Checks if the value of the field is `Div107`"]
    #[inline(always)]
    pub fn is_div107(&self) -> bool {
        *self == PLLN_A::Div107
    }
    #[doc = "Checks if the value of the field is `Div108`"]
    #[inline(always)]
    pub fn is_div108(&self) -> bool {
        *self == PLLN_A::Div108
    }
    #[doc = "Checks if the value of the field is `Div109`"]
    #[inline(always)]
    pub fn is_div109(&self) -> bool {
        *self == PLLN_A::Div109
    }
    #[doc = "Checks if the value of the field is `Div110`"]
    #[inline(always)]
    pub fn is_div110(&self) -> bool {
        *self == PLLN_A::Div110
    }
    #[doc = "Checks if the value of the field is `Div111`"]
    #[inline(always)]
    pub fn is_div111(&self) -> bool {
        *self == PLLN_A::Div111
    }
    #[doc = "Checks if the value of the field is `Div112`"]
    #[inline(always)]
    pub fn is_div112(&self) -> bool {
        *self == PLLN_A::Div112
    }
    #[doc = "Checks if the value of the field is `Div113`"]
    #[inline(always)]
    pub fn is_div113(&self) -> bool {
        *self == PLLN_A::Div113
    }
    #[doc = "Checks if the value of the field is `Div114`"]
    #[inline(always)]
    pub fn is_div114(&self) -> bool {
        *self == PLLN_A::Div114
    }
    #[doc = "Checks if the value of the field is `Div115`"]
    #[inline(always)]
    pub fn is_div115(&self) -> bool {
        *self == PLLN_A::Div115
    }
    #[doc = "Checks if the value of the field is `Div116`"]
    #[inline(always)]
    pub fn is_div116(&self) -> bool {
        *self == PLLN_A::Div116
    }
    #[doc = "Checks if the value of the field is `Div117`"]
    #[inline(always)]
    pub fn is_div117(&self) -> bool {
        *self == PLLN_A::Div117
    }
    #[doc = "Checks if the value of the field is `Div118`"]
    #[inline(always)]
    pub fn is_div118(&self) -> bool {
        *self == PLLN_A::Div118
    }
    #[doc = "Checks if the value of the field is `Div119`"]
    #[inline(always)]
    pub fn is_div119(&self) -> bool {
        *self == PLLN_A::Div119
    }
    #[doc = "Checks if the value of the field is `Div120`"]
    #[inline(always)]
    pub fn is_div120(&self) -> bool {
        *self == PLLN_A::Div120
    }
    #[doc = "Checks if the value of the field is `Div121`"]
    #[inline(always)]
    pub fn is_div121(&self) -> bool {
        *self == PLLN_A::Div121
    }
    #[doc = "Checks if the value of the field is `Div122`"]
    #[inline(always)]
    pub fn is_div122(&self) -> bool {
        *self == PLLN_A::Div122
    }
    #[doc = "Checks if the value of the field is `Div123`"]
    #[inline(always)]
    pub fn is_div123(&self) -> bool {
        *self == PLLN_A::Div123
    }
    #[doc = "Checks if the value of the field is `Div124`"]
    #[inline(always)]
    pub fn is_div124(&self) -> bool {
        *self == PLLN_A::Div124
    }
    #[doc = "Checks if the value of the field is `Div125`"]
    #[inline(always)]
    pub fn is_div125(&self) -> bool {
        *self == PLLN_A::Div125
    }
    #[doc = "Checks if the value of the field is `Div126`"]
    #[inline(always)]
    pub fn is_div126(&self) -> bool {
        *self == PLLN_A::Div126
    }
    #[doc = "Checks if the value of the field is `Div127`"]
    #[inline(always)]
    pub fn is_div127(&self) -> bool {
        *self == PLLN_A::Div127
    }
}
#[doc = "Field `PLLN` writer - Main PLL multiplication factor for VCO"]
pub type PLLN_W<'a, const O: u8> = crate::FieldWriter<'a, u32, PLLCFGR_SPEC, u8, PLLN_A, 7, O>;
impl<'a, const O: u8> PLLN_W<'a, O> {
    #[doc = "pll_n_ck = vco_ck / 8"]
    #[inline(always)]
    pub fn div8(self) -> &'a mut W {
        self.variant(PLLN_A::Div8)
    }
    #[doc = "pll_n_ck = vco_ck / 9"]
    #[inline(always)]
    pub fn div9(self) -> &'a mut W {
        self.variant(PLLN_A::Div9)
    }
    #[doc = "pll_n_ck = vco_ck / 10"]
    #[inline(always)]
    pub fn div10(self) -> &'a mut W {
        self.variant(PLLN_A::Div10)
    }
    #[doc = "pll_n_ck = vco_ck / 11"]
    #[inline(always)]
    pub fn div11(self) -> &'a mut W {
        self.variant(PLLN_A::Div11)
    }
    #[doc = "pll_n_ck = vco_ck / 12"]
    #[inline(always)]
    pub fn div12(self) -> &'a mut W {
        self.variant(PLLN_A::Div12)
    }
    #[doc = "pll_n_ck = vco_ck / 13"]
    #[inline(always)]
    pub fn div13(self) -> &'a mut W {
        self.variant(PLLN_A::Div13)
    }
    #[doc = "pll_n_ck = vco_ck / 14"]
    #[inline(always)]
    pub fn div14(self) -> &'a mut W {
        self.variant(PLLN_A::Div14)
    }
    #[doc = "pll_n_ck = vco_ck / 15"]
    #[inline(always)]
    pub fn div15(self) -> &'a mut W {
        self.variant(PLLN_A::Div15)
    }
    #[doc = "pll_n_ck = vco_ck / 16"]
    #[inline(always)]
    pub fn div16(self) -> &'a mut W {
        self.variant(PLLN_A::Div16)
    }
    #[doc = "pll_n_ck = vco_ck / 17"]
    #[inline(always)]
    pub fn div17(self) -> &'a mut W {
        self.variant(PLLN_A::Div17)
    }
    #[doc = "pll_n_ck = vco_ck / 18"]
    #[inline(always)]
    pub fn div18(self) -> &'a mut W {
        self.variant(PLLN_A::Div18)
    }
    #[doc = "pll_n_ck = vco_ck / 19"]
    #[inline(always)]
    pub fn div19(self) -> &'a mut W {
        self.variant(PLLN_A::Div19)
    }
    #[doc = "pll_n_ck = vco_ck / 20"]
    #[inline(always)]
    pub fn div20(self) -> &'a mut W {
        self.variant(PLLN_A::Div20)
    }
    #[doc = "pll_n_ck = vco_ck / 21"]
    #[inline(always)]
    pub fn div21(self) -> &'a mut W {
        self.variant(PLLN_A::Div21)
    }
    #[doc = "pll_n_ck = vco_ck / 22"]
    #[inline(always)]
    pub fn div22(self) -> &'a mut W {
        self.variant(PLLN_A::Div22)
    }
    #[doc = "pll_n_ck = vco_ck / 23"]
    #[inline(always)]
    pub fn div23(self) -> &'a mut W {
        self.variant(PLLN_A::Div23)
    }
    #[doc = "pll_n_ck = vco_ck / 24"]
    #[inline(always)]
    pub fn div24(self) -> &'a mut W {
        self.variant(PLLN_A::Div24)
    }
    #[doc = "pll_n_ck = vco_ck / 25"]
    #[inline(always)]
    pub fn div25(self) -> &'a mut W {
        self.variant(PLLN_A::Div25)
    }
    #[doc = "pll_n_ck = vco_ck / 26"]
    #[inline(always)]
    pub fn div26(self) -> &'a mut W {
        self.variant(PLLN_A::Div26)
    }
    #[doc = "pll_n_ck = vco_ck / 27"]
    #[inline(always)]
    pub fn div27(self) -> &'a mut W {
        self.variant(PLLN_A::Div27)
    }
    #[doc = "pll_n_ck = vco_ck / 28"]
    #[inline(always)]
    pub fn div28(self) -> &'a mut W {
        self.variant(PLLN_A::Div28)
    }
    #[doc = "pll_n_ck = vco_ck / 29"]
    #[inline(always)]
    pub fn div29(self) -> &'a mut W {
        self.variant(PLLN_A::Div29)
    }
    #[doc = "pll_n_ck = vco_ck / 30"]
    #[inline(always)]
    pub fn div30(self) -> &'a mut W {
        self.variant(PLLN_A::Div30)
    }
    #[doc = "pll_n_ck = vco_ck / 31"]
    #[inline(always)]
    pub fn div31(self) -> &'a mut W {
        self.variant(PLLN_A::Div31)
    }
    #[doc = "pll_n_ck = vco_ck / 32"]
    #[inline(always)]
    pub fn div32(self) -> &'a mut W {
        self.variant(PLLN_A::Div32)
    }
    #[doc = "pll_n_ck = vco_ck / 33"]
    #[inline(always)]
    pub fn div33(self) -> &'a mut W {
        self.variant(PLLN_A::Div33)
    }
    #[doc = "pll_n_ck = vco_ck / 34"]
    #[inline(always)]
    pub fn div34(self) -> &'a mut W {
        self.variant(PLLN_A::Div34)
    }
    #[doc = "pll_n_ck = vco_ck / 35"]
    #[inline(always)]
    pub fn div35(self) -> &'a mut W {
        self.variant(PLLN_A::Div35)
    }
    #[doc = "pll_n_ck = vco_ck / 36"]
    #[inline(always)]
    pub fn div36(self) -> &'a mut W {
        self.variant(PLLN_A::Div36)
    }
    #[doc = "pll_n_ck = vco_ck / 37"]
    #[inline(always)]
    pub fn div37(self) -> &'a mut W {
        self.variant(PLLN_A::Div37)
    }
    #[doc = "pll_n_ck = vco_ck / 38"]
    #[inline(always)]
    pub fn div38(self) -> &'a mut W {
        self.variant(PLLN_A::Div38)
    }
    #[doc = "pll_n_ck = vco_ck / 39"]
    #[inline(always)]
    pub fn div39(self) -> &'a mut W {
        self.variant(PLLN_A::Div39)
    }
    #[doc = "pll_n_ck = vco_ck / 40"]
    #[inline(always)]
    pub fn div40(self) -> &'a mut W {
        self.variant(PLLN_A::Div40)
    }
    #[doc = "pll_n_ck = vco_ck / 41"]
    #[inline(always)]
    pub fn div41(self) -> &'a mut W {
        self.variant(PLLN_A::Div41)
    }
    #[doc = "pll_n_ck = vco_ck / 42"]
    #[inline(always)]
    pub fn div42(self) -> &'a mut W {
        self.variant(PLLN_A::Div42)
    }
    #[doc = "pll_n_ck = vco_ck / 43"]
    #[inline(always)]
    pub fn div43(self) -> &'a mut W {
        self.variant(PLLN_A::Div43)
    }
    #[doc = "pll_n_ck = vco_ck / 44"]
    #[inline(always)]
    pub fn div44(self) -> &'a mut W {
        self.variant(PLLN_A::Div44)
    }
    #[doc = "pll_n_ck = vco_ck / 45"]
    #[inline(always)]
    pub fn div45(self) -> &'a mut W {
        self.variant(PLLN_A::Div45)
    }
    #[doc = "pll_n_ck = vco_ck / 46"]
    #[inline(always)]
    pub fn div46(self) -> &'a mut W {
        self.variant(PLLN_A::Div46)
    }
    #[doc = "pll_n_ck = vco_ck / 47"]
    #[inline(always)]
    pub fn div47(self) -> &'a mut W {
        self.variant(PLLN_A::Div47)
    }
    #[doc = "pll_n_ck = vco_ck / 48"]
    #[inline(always)]
    pub fn div48(self) -> &'a mut W {
        self.variant(PLLN_A::Div48)
    }
    #[doc = "pll_n_ck = vco_ck / 49"]
    #[inline(always)]
    pub fn div49(self) -> &'a mut W {
        self.variant(PLLN_A::Div49)
    }
    #[doc = "pll_n_ck = vco_ck / 50"]
    #[inline(always)]
    pub fn div50(self) -> &'a mut W {
        self.variant(PLLN_A::Div50)
    }
    #[doc = "pll_n_ck = vco_ck / 51"]
    #[inline(always)]
    pub fn div51(self) -> &'a mut W {
        self.variant(PLLN_A::Div51)
    }
    #[doc = "pll_n_ck = vco_ck / 52"]
    #[inline(always)]
    pub fn div52(self) -> &'a mut W {
        self.variant(PLLN_A::Div52)
    }
    #[doc = "pll_n_ck = vco_ck / 53"]
    #[inline(always)]
    pub fn div53(self) -> &'a mut W {
        self.variant(PLLN_A::Div53)
    }
    #[doc = "pll_n_ck = vco_ck / 54"]
    #[inline(always)]
    pub fn div54(self) -> &'a mut W {
        self.variant(PLLN_A::Div54)
    }
    #[doc = "pll_n_ck = vco_ck / 55"]
    #[inline(always)]
    pub fn div55(self) -> &'a mut W {
        self.variant(PLLN_A::Div55)
    }
    #[doc = "pll_n_ck = vco_ck / 56"]
    #[inline(always)]
    pub fn div56(self) -> &'a mut W {
        self.variant(PLLN_A::Div56)
    }
    #[doc = "pll_n_ck = vco_ck / 57"]
    #[inline(always)]
    pub fn div57(self) -> &'a mut W {
        self.variant(PLLN_A::Div57)
    }
    #[doc = "pll_n_ck = vco_ck / 58"]
    #[inline(always)]
    pub fn div58(self) -> &'a mut W {
        self.variant(PLLN_A::Div58)
    }
    #[doc = "pll_n_ck = vco_ck / 59"]
    #[inline(always)]
    pub fn div59(self) -> &'a mut W {
        self.variant(PLLN_A::Div59)
    }
    #[doc = "pll_n_ck = vco_ck / 60"]
    #[inline(always)]
    pub fn div60(self) -> &'a mut W {
        self.variant(PLLN_A::Div60)
    }
    #[doc = "pll_n_ck = vco_ck / 61"]
    #[inline(always)]
    pub fn div61(self) -> &'a mut W {
        self.variant(PLLN_A::Div61)
    }
    #[doc = "pll_n_ck = vco_ck / 62"]
    #[inline(always)]
    pub fn div62(self) -> &'a mut W {
        self.variant(PLLN_A::Div62)
    }
    #[doc = "pll_n_ck = vco_ck / 63"]
    #[inline(always)]
    pub fn div63(self) -> &'a mut W {
        self.variant(PLLN_A::Div63)
    }
    #[doc = "pll_n_ck = vco_ck / 64"]
    #[inline(always)]
    pub fn div64(self) -> &'a mut W {
        self.variant(PLLN_A::Div64)
    }
    #[doc = "pll_n_ck = vco_ck / 65"]
    #[inline(always)]
    pub fn div65(self) -> &'a mut W {
        self.variant(PLLN_A::Div65)
    }
    #[doc = "pll_n_ck = vco_ck / 66"]
    #[inline(always)]
    pub fn div66(self) -> &'a mut W {
        self.variant(PLLN_A::Div66)
    }
    #[doc = "pll_n_ck = vco_ck / 67"]
    #[inline(always)]
    pub fn div67(self) -> &'a mut W {
        self.variant(PLLN_A::Div67)
    }
    #[doc = "pll_n_ck = vco_ck / 68"]
    #[inline(always)]
    pub fn div68(self) -> &'a mut W {
        self.variant(PLLN_A::Div68)
    }
    #[doc = "pll_n_ck = vco_ck / 69"]
    #[inline(always)]
    pub fn div69(self) -> &'a mut W {
        self.variant(PLLN_A::Div69)
    }
    #[doc = "pll_n_ck = vco_ck / 70"]
    #[inline(always)]
    pub fn div70(self) -> &'a mut W {
        self.variant(PLLN_A::Div70)
    }
    #[doc = "pll_n_ck = vco_ck / 71"]
    #[inline(always)]
    pub fn div71(self) -> &'a mut W {
        self.variant(PLLN_A::Div71)
    }
    #[doc = "pll_n_ck = vco_ck / 72"]
    #[inline(always)]
    pub fn div72(self) -> &'a mut W {
        self.variant(PLLN_A::Div72)
    }
    #[doc = "pll_n_ck = vco_ck / 73"]
    #[inline(always)]
    pub fn div73(self) -> &'a mut W {
        self.variant(PLLN_A::Div73)
    }
    #[doc = "pll_n_ck = vco_ck / 74"]
    #[inline(always)]
    pub fn div74(self) -> &'a mut W {
        self.variant(PLLN_A::Div74)
    }
    #[doc = "pll_n_ck = vco_ck / 75"]
    #[inline(always)]
    pub fn div75(self) -> &'a mut W {
        self.variant(PLLN_A::Div75)
    }
    #[doc = "pll_n_ck = vco_ck / 76"]
    #[inline(always)]
    pub fn div76(self) -> &'a mut W {
        self.variant(PLLN_A::Div76)
    }
    #[doc = "pll_n_ck = vco_ck / 77"]
    #[inline(always)]
    pub fn div77(self) -> &'a mut W {
        self.variant(PLLN_A::Div77)
    }
    #[doc = "pll_n_ck = vco_ck / 78"]
    #[inline(always)]
    pub fn div78(self) -> &'a mut W {
        self.variant(PLLN_A::Div78)
    }
    #[doc = "pll_n_ck = vco_ck / 79"]
    #[inline(always)]
    pub fn div79(self) -> &'a mut W {
        self.variant(PLLN_A::Div79)
    }
    #[doc = "pll_n_ck = vco_ck / 80"]
    #[inline(always)]
    pub fn div80(self) -> &'a mut W {
        self.variant(PLLN_A::Div80)
    }
    #[doc = "pll_n_ck = vco_ck / 81"]
    #[inline(always)]
    pub fn div81(self) -> &'a mut W {
        self.variant(PLLN_A::Div81)
    }
    #[doc = "pll_n_ck = vco_ck / 82"]
    #[inline(always)]
    pub fn div82(self) -> &'a mut W {
        self.variant(PLLN_A::Div82)
    }
    #[doc = "pll_n_ck = vco_ck / 83"]
    #[inline(always)]
    pub fn div83(self) -> &'a mut W {
        self.variant(PLLN_A::Div83)
    }
    #[doc = "pll_n_ck = vco_ck / 84"]
    #[inline(always)]
    pub fn div84(self) -> &'a mut W {
        self.variant(PLLN_A::Div84)
    }
    #[doc = "pll_n_ck = vco_ck / 85"]
    #[inline(always)]
    pub fn div85(self) -> &'a mut W {
        self.variant(PLLN_A::Div85)
    }
    #[doc = "pll_n_ck = vco_ck / 86"]
    #[inline(always)]
    pub fn div86(self) -> &'a mut W {
        self.variant(PLLN_A::Div86)
    }
    #[doc = "pll_n_ck = vco_ck / 87"]
    #[inline(always)]
    pub fn div87(self) -> &'a mut W {
        self.variant(PLLN_A::Div87)
    }
    #[doc = "pll_n_ck = vco_ck / 88"]
    #[inline(always)]
    pub fn div88(self) -> &'a mut W {
        self.variant(PLLN_A::Div88)
    }
    #[doc = "pll_n_ck = vco_ck / 89"]
    #[inline(always)]
    pub fn div89(self) -> &'a mut W {
        self.variant(PLLN_A::Div89)
    }
    #[doc = "pll_n_ck = vco_ck / 90"]
    #[inline(always)]
    pub fn div90(self) -> &'a mut W {
        self.variant(PLLN_A::Div90)
    }
    #[doc = "pll_n_ck = vco_ck / 91"]
    #[inline(always)]
    pub fn div91(self) -> &'a mut W {
        self.variant(PLLN_A::Div91)
    }
    #[doc = "pll_n_ck = vco_ck / 92"]
    #[inline(always)]
    pub fn div92(self) -> &'a mut W {
        self.variant(PLLN_A::Div92)
    }
    #[doc = "pll_n_ck = vco_ck / 93"]
    #[inline(always)]
    pub fn div93(self) -> &'a mut W {
        self.variant(PLLN_A::Div93)
    }
    #[doc = "pll_n_ck = vco_ck / 94"]
    #[inline(always)]
    pub fn div94(self) -> &'a mut W {
        self.variant(PLLN_A::Div94)
    }
    #[doc = "pll_n_ck = vco_ck / 95"]
    #[inline(always)]
    pub fn div95(self) -> &'a mut W {
        self.variant(PLLN_A::Div95)
    }
    #[doc = "pll_n_ck = vco_ck / 96"]
    #[inline(always)]
    pub fn div96(self) -> &'a mut W {
        self.variant(PLLN_A::Div96)
    }
    #[doc = "pll_n_ck = vco_ck / 97"]
    #[inline(always)]
    pub fn div97(self) -> &'a mut W {
        self.variant(PLLN_A::Div97)
    }
    #[doc = "pll_n_ck = vco_ck / 98"]
    #[inline(always)]
    pub fn div98(self) -> &'a mut W {
        self.variant(PLLN_A::Div98)
    }
    #[doc = "pll_n_ck = vco_ck / 99"]
    #[inline(always)]
    pub fn div99(self) -> &'a mut W {
        self.variant(PLLN_A::Div99)
    }
    #[doc = "pll_n_ck = vco_ck / 100"]
    #[inline(always)]
    pub fn div100(self) -> &'a mut W {
        self.variant(PLLN_A::Div100)
    }
    #[doc = "pll_n_ck = vco_ck / 101"]
    #[inline(always)]
    pub fn div101(self) -> &'a mut W {
        self.variant(PLLN_A::Div101)
    }
    #[doc = "pll_n_ck = vco_ck / 102"]
    #[inline(always)]
    pub fn div102(self) -> &'a mut W {
        self.variant(PLLN_A::Div102)
    }
    #[doc = "pll_n_ck = vco_ck / 103"]
    #[inline(always)]
    pub fn div103(self) -> &'a mut W {
        self.variant(PLLN_A::Div103)
    }
    #[doc = "pll_n_ck = vco_ck / 104"]
    #[inline(always)]
    pub fn div104(self) -> &'a mut W {
        self.variant(PLLN_A::Div104)
    }
    #[doc = "pll_n_ck = vco_ck / 105"]
    #[inline(always)]
    pub fn div105(self) -> &'a mut W {
        self.variant(PLLN_A::Div105)
    }
    #[doc = "pll_n_ck = vco_ck / 106"]
    #[inline(always)]
    pub fn div106(self) -> &'a mut W {
        self.variant(PLLN_A::Div106)
    }
    #[doc = "pll_n_ck = vco_ck / 107"]
    #[inline(always)]
    pub fn div107(self) -> &'a mut W {
        self.variant(PLLN_A::Div107)
    }
    #[doc = "pll_n_ck = vco_ck / 108"]
    #[inline(always)]
    pub fn div108(self) -> &'a mut W {
        self.variant(PLLN_A::Div108)
    }
    #[doc = "pll_n_ck = vco_ck / 109"]
    #[inline(always)]
    pub fn div109(self) -> &'a mut W {
        self.variant(PLLN_A::Div109)
    }
    #[doc = "pll_n_ck = vco_ck / 110"]
    #[inline(always)]
    pub fn div110(self) -> &'a mut W {
        self.variant(PLLN_A::Div110)
    }
    #[doc = "pll_n_ck = vco_ck / 111"]
    #[inline(always)]
    pub fn div111(self) -> &'a mut W {
        self.variant(PLLN_A::Div111)
    }
    #[doc = "pll_n_ck = vco_ck / 112"]
    #[inline(always)]
    pub fn div112(self) -> &'a mut W {
        self.variant(PLLN_A::Div112)
    }
    #[doc = "pll_n_ck = vco_ck / 113"]
    #[inline(always)]
    pub fn div113(self) -> &'a mut W {
        self.variant(PLLN_A::Div113)
    }
    #[doc = "pll_n_ck = vco_ck / 114"]
    #[inline(always)]
    pub fn div114(self) -> &'a mut W {
        self.variant(PLLN_A::Div114)
    }
    #[doc = "pll_n_ck = vco_ck / 115"]
    #[inline(always)]
    pub fn div115(self) -> &'a mut W {
        self.variant(PLLN_A::Div115)
    }
    #[doc = "pll_n_ck = vco_ck / 116"]
    #[inline(always)]
    pub fn div116(self) -> &'a mut W {
        self.variant(PLLN_A::Div116)
    }
    #[doc = "pll_n_ck = vco_ck / 117"]
    #[inline(always)]
    pub fn div117(self) -> &'a mut W {
        self.variant(PLLN_A::Div117)
    }
    #[doc = "pll_n_ck = vco_ck / 118"]
    #[inline(always)]
    pub fn div118(self) -> &'a mut W {
        self.variant(PLLN_A::Div118)
    }
    #[doc = "pll_n_ck = vco_ck / 119"]
    #[inline(always)]
    pub fn div119(self) -> &'a mut W {
        self.variant(PLLN_A::Div119)
    }
    #[doc = "pll_n_ck = vco_ck / 120"]
    #[inline(always)]
    pub fn div120(self) -> &'a mut W {
        self.variant(PLLN_A::Div120)
    }
    #[doc = "pll_n_ck = vco_ck / 121"]
    #[inline(always)]
    pub fn div121(self) -> &'a mut W {
        self.variant(PLLN_A::Div121)
    }
    #[doc = "pll_n_ck = vco_ck / 122"]
    #[inline(always)]
    pub fn div122(self) -> &'a mut W {
        self.variant(PLLN_A::Div122)
    }
    #[doc = "pll_n_ck = vco_ck / 123"]
    #[inline(always)]
    pub fn div123(self) -> &'a mut W {
        self.variant(PLLN_A::Div123)
    }
    #[doc = "pll_n_ck = vco_ck / 124"]
    #[inline(always)]
    pub fn div124(self) -> &'a mut W {
        self.variant(PLLN_A::Div124)
    }
    #[doc = "pll_n_ck = vco_ck / 125"]
    #[inline(always)]
    pub fn div125(self) -> &'a mut W {
        self.variant(PLLN_A::Div125)
    }
    #[doc = "pll_n_ck = vco_ck / 126"]
    #[inline(always)]
    pub fn div126(self) -> &'a mut W {
        self.variant(PLLN_A::Div126)
    }
    #[doc = "pll_n_ck = vco_ck / 127"]
    #[inline(always)]
    pub fn div127(self) -> &'a mut W {
        self.variant(PLLN_A::Div127)
    }
}
#[doc = "Field `PLLPEN` reader - Main PLL PLLSAI3CLK output enable"]
pub type PLLPEN_R = crate::BitReader<bool>;
#[doc = "Field `PLLPEN` writer - Main PLL PLLSAI3CLK output enable"]
pub type PLLPEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, PLLCFGR_SPEC, bool, O>;
#[doc = "Field `PLLP` reader - Main PLL division factor for PLLSAI3CLK (SAI1 and SAI2 clock)"]
pub type PLLP_R = crate::BitReader<PLLP_A>;
#[doc = "Main PLL division factor for PLLSAI3CLK (SAI1 and SAI2 clock)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PLLP_A {
    #[doc = "0: pll_p_ck = vco_ck / 7"]
    Div7 = 0,
    #[doc = "1: pll_p_ck = vco_ck / 17"]
    Div17 = 1,
}
impl From<PLLP_A> for bool {
    #[inline(always)]
    fn from(variant: PLLP_A) -> Self {
        variant as u8 != 0
    }
}
impl PLLP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PLLP_A {
        match self.bits {
            false => PLLP_A::Div7,
            true => PLLP_A::Div17,
        }
    }
    #[doc = "Checks if the value of the field is `Div7`"]
    #[inline(always)]
    pub fn is_div7(&self) -> bool {
        *self == PLLP_A::Div7
    }
    #[doc = "Checks if the value of the field is `Div17`"]
    #[inline(always)]
    pub fn is_div17(&self) -> bool {
        *self == PLLP_A::Div17
    }
}
#[doc = "Field `PLLP` writer - Main PLL division factor for PLLSAI3CLK (SAI1 and SAI2 clock)"]
pub type PLLP_W<'a, const O: u8> = crate::BitWriter<'a, u32, PLLCFGR_SPEC, PLLP_A, O>;
impl<'a, const O: u8> PLLP_W<'a, O> {
    #[doc = "pll_p_ck = vco_ck / 7"]
    #[inline(always)]
    pub fn div7(self) -> &'a mut W {
        self.variant(PLLP_A::Div7)
    }
    #[doc = "pll_p_ck = vco_ck / 17"]
    #[inline(always)]
    pub fn div17(self) -> &'a mut W {
        self.variant(PLLP_A::Div17)
    }
}
#[doc = "Field `PLLQEN` reader - Main PLL PLLUSB1CLK output enable"]
pub type PLLQEN_R = crate::BitReader<bool>;
#[doc = "Field `PLLQEN` writer - Main PLL PLLUSB1CLK output enable"]
pub type PLLQEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, PLLCFGR_SPEC, bool, O>;
#[doc = "Field `PLLQ` reader - Main PLL division factor for PLLUSB1CLK(48 MHz clock)"]
pub type PLLQ_R = crate::FieldReader<u8, PLLQ_A>;
#[doc = "Main PLL division factor for PLLUSB1CLK(48 MHz clock)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PLLQ_A {
    #[doc = "0: pll_q_ck = vco_ck / 2"]
    Div2 = 0,
    #[doc = "1: pll_q_ck = vco_ck / 4"]
    Div4 = 1,
    #[doc = "2: pll_q_ck = vco_ck / 6"]
    Div6 = 2,
    #[doc = "3: pll_q_ck = vco_ck / 8"]
    Div8 = 3,
}
impl From<PLLQ_A> for u8 {
    #[inline(always)]
    fn from(variant: PLLQ_A) -> Self {
        variant as _
    }
}
impl PLLQ_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PLLQ_A {
        match self.bits {
            0 => PLLQ_A::Div2,
            1 => PLLQ_A::Div4,
            2 => PLLQ_A::Div6,
            3 => PLLQ_A::Div8,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `Div2`"]
    #[inline(always)]
    pub fn is_div2(&self) -> bool {
        *self == PLLQ_A::Div2
    }
    #[doc = "Checks if the value of the field is `Div4`"]
    #[inline(always)]
    pub fn is_div4(&self) -> bool {
        *self == PLLQ_A::Div4
    }
    #[doc = "Checks if the value of the field is `Div6`"]
    #[inline(always)]
    pub fn is_div6(&self) -> bool {
        *self == PLLQ_A::Div6
    }
    #[doc = "Checks if the value of the field is `Div8`"]
    #[inline(always)]
    pub fn is_div8(&self) -> bool {
        *self == PLLQ_A::Div8
    }
}
#[doc = "Field `PLLQ` writer - Main PLL division factor for PLLUSB1CLK(48 MHz clock)"]
pub type PLLQ_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, PLLCFGR_SPEC, u8, PLLQ_A, 2, O>;
impl<'a, const O: u8> PLLQ_W<'a, O> {
    #[doc = "pll_q_ck = vco_ck / 2"]
    #[inline(always)]
    pub fn div2(self) -> &'a mut W {
        self.variant(PLLQ_A::Div2)
    }
    #[doc = "pll_q_ck = vco_ck / 4"]
    #[inline(always)]
    pub fn div4(self) -> &'a mut W {
        self.variant(PLLQ_A::Div4)
    }
    #[doc = "pll_q_ck = vco_ck / 6"]
    #[inline(always)]
    pub fn div6(self) -> &'a mut W {
        self.variant(PLLQ_A::Div6)
    }
    #[doc = "pll_q_ck = vco_ck / 8"]
    #[inline(always)]
    pub fn div8(self) -> &'a mut W {
        self.variant(PLLQ_A::Div8)
    }
}
#[doc = "Field `PLLREN` reader - Main PLL PLLCLK output enable"]
pub type PLLREN_R = crate::BitReader<bool>;
#[doc = "Field `PLLREN` writer - Main PLL PLLCLK output enable"]
pub type PLLREN_W<'a, const O: u8> = crate::BitWriter<'a, u32, PLLCFGR_SPEC, bool, O>;
#[doc = "Field `PLLR` reader - Main PLL division factor for PLLCLK (system clock)"]
pub type PLLR_R = crate::FieldReader<u8, PLLR_A>;
#[doc = "Main PLL division factor for PLLCLK (system clock)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PLLR_A {
    #[doc = "0: pll_r_ck = vco_ck / 2"]
    Div2 = 0,
    #[doc = "1: pll_r_ck = vco_ck / 4"]
    Div4 = 1,
    #[doc = "2: pll_r_ck = vco_ck / 6"]
    Div6 = 2,
    #[doc = "3: pll_r_ck = vco_ck / 8"]
    Div8 = 3,
}
impl From<PLLR_A> for u8 {
    #[inline(always)]
    fn from(variant: PLLR_A) -> Self {
        variant as _
    }
}
impl PLLR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PLLR_A {
        match self.bits {
            0 => PLLR_A::Div2,
            1 => PLLR_A::Div4,
            2 => PLLR_A::Div6,
            3 => PLLR_A::Div8,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `Div2`"]
    #[inline(always)]
    pub fn is_div2(&self) -> bool {
        *self == PLLR_A::Div2
    }
    #[doc = "Checks if the value of the field is `Div4`"]
    #[inline(always)]
    pub fn is_div4(&self) -> bool {
        *self == PLLR_A::Div4
    }
    #[doc = "Checks if the value of the field is `Div6`"]
    #[inline(always)]
    pub fn is_div6(&self) -> bool {
        *self == PLLR_A::Div6
    }
    #[doc = "Checks if the value of the field is `Div8`"]
    #[inline(always)]
    pub fn is_div8(&self) -> bool {
        *self == PLLR_A::Div8
    }
}
#[doc = "Field `PLLR` writer - Main PLL division factor for PLLCLK (system clock)"]
pub type PLLR_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, PLLCFGR_SPEC, u8, PLLR_A, 2, O>;
impl<'a, const O: u8> PLLR_W<'a, O> {
    #[doc = "pll_r_ck = vco_ck / 2"]
    #[inline(always)]
    pub fn div2(self) -> &'a mut W {
        self.variant(PLLR_A::Div2)
    }
    #[doc = "pll_r_ck = vco_ck / 4"]
    #[inline(always)]
    pub fn div4(self) -> &'a mut W {
        self.variant(PLLR_A::Div4)
    }
    #[doc = "pll_r_ck = vco_ck / 6"]
    #[inline(always)]
    pub fn div6(self) -> &'a mut W {
        self.variant(PLLR_A::Div6)
    }
    #[doc = "pll_r_ck = vco_ck / 8"]
    #[inline(always)]
    pub fn div8(self) -> &'a mut W {
        self.variant(PLLR_A::Div8)
    }
}
#[doc = "Field `PLLPDIV` reader - Main PLL division factor for PLLSAI2CLK"]
pub type PLLPDIV_R = crate::FieldReader<u8, PLLPDIV_A>;
#[doc = "Main PLL division factor for PLLSAI2CLK\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PLLPDIV_A {
    #[doc = "0: pll_p_ck is controlled by PLLP"]
    Pllp = 0,
    #[doc = "2: pll_p_ck = vco_ck / 2"]
    Div2 = 2,
    #[doc = "3: pll_p_ck = vco_ck / 3"]
    Div3 = 3,
    #[doc = "4: pll_p_ck = vco_ck / 4"]
    Div4 = 4,
    #[doc = "5: pll_p_ck = vco_ck / 5"]
    Div5 = 5,
    #[doc = "6: pll_p_ck = vco_ck / 6"]
    Div6 = 6,
    #[doc = "7: pll_p_ck = vco_ck / 7"]
    Div7 = 7,
    #[doc = "8: pll_p_ck = vco_ck / 8"]
    Div8 = 8,
    #[doc = "9: pll_p_ck = vco_ck / 9"]
    Div9 = 9,
    #[doc = "10: pll_p_ck = vco_ck / 10"]
    Div10 = 10,
    #[doc = "11: pll_p_ck = vco_ck / 11"]
    Div11 = 11,
    #[doc = "12: pll_p_ck = vco_ck / 12"]
    Div12 = 12,
    #[doc = "13: pll_p_ck = vco_ck / 13"]
    Div13 = 13,
    #[doc = "14: pll_p_ck = vco_ck / 14"]
    Div14 = 14,
    #[doc = "15: pll_p_ck = vco_ck / 15"]
    Div15 = 15,
    #[doc = "16: pll_p_ck = vco_ck / 16"]
    Div16 = 16,
    #[doc = "17: pll_p_ck = vco_ck / 17"]
    Div17 = 17,
    #[doc = "18: pll_p_ck = vco_ck / 18"]
    Div18 = 18,
    #[doc = "19: pll_p_ck = vco_ck / 19"]
    Div19 = 19,
    #[doc = "20: pll_p_ck = vco_ck / 20"]
    Div20 = 20,
    #[doc = "21: pll_p_ck = vco_ck / 21"]
    Div21 = 21,
    #[doc = "22: pll_p_ck = vco_ck / 22"]
    Div22 = 22,
    #[doc = "23: pll_p_ck = vco_ck / 23"]
    Div23 = 23,
    #[doc = "24: pll_p_ck = vco_ck / 24"]
    Div24 = 24,
    #[doc = "25: pll_p_ck = vco_ck / 25"]
    Div25 = 25,
    #[doc = "26: pll_p_ck = vco_ck / 26"]
    Div26 = 26,
    #[doc = "27: pll_p_ck = vco_ck / 27"]
    Div27 = 27,
    #[doc = "28: pll_p_ck = vco_ck / 28"]
    Div28 = 28,
    #[doc = "29: pll_p_ck = vco_ck / 29"]
    Div29 = 29,
    #[doc = "30: pll_p_ck = vco_ck / 30"]
    Div30 = 30,
    #[doc = "31: pll_p_ck = vco_ck / 31"]
    Div31 = 31,
}
impl From<PLLPDIV_A> for u8 {
    #[inline(always)]
    fn from(variant: PLLPDIV_A) -> Self {
        variant as _
    }
}
impl PLLPDIV_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<PLLPDIV_A> {
        match self.bits {
            0 => Some(PLLPDIV_A::Pllp),
            2 => Some(PLLPDIV_A::Div2),
            3 => Some(PLLPDIV_A::Div3),
            4 => Some(PLLPDIV_A::Div4),
            5 => Some(PLLPDIV_A::Div5),
            6 => Some(PLLPDIV_A::Div6),
            7 => Some(PLLPDIV_A::Div7),
            8 => Some(PLLPDIV_A::Div8),
            9 => Some(PLLPDIV_A::Div9),
            10 => Some(PLLPDIV_A::Div10),
            11 => Some(PLLPDIV_A::Div11),
            12 => Some(PLLPDIV_A::Div12),
            13 => Some(PLLPDIV_A::Div13),
            14 => Some(PLLPDIV_A::Div14),
            15 => Some(PLLPDIV_A::Div15),
            16 => Some(PLLPDIV_A::Div16),
            17 => Some(PLLPDIV_A::Div17),
            18 => Some(PLLPDIV_A::Div18),
            19 => Some(PLLPDIV_A::Div19),
            20 => Some(PLLPDIV_A::Div20),
            21 => Some(PLLPDIV_A::Div21),
            22 => Some(PLLPDIV_A::Div22),
            23 => Some(PLLPDIV_A::Div23),
            24 => Some(PLLPDIV_A::Div24),
            25 => Some(PLLPDIV_A::Div25),
            26 => Some(PLLPDIV_A::Div26),
            27 => Some(PLLPDIV_A::Div27),
            28 => Some(PLLPDIV_A::Div28),
            29 => Some(PLLPDIV_A::Div29),
            30 => Some(PLLPDIV_A::Div30),
            31 => Some(PLLPDIV_A::Div31),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `Pllp`"]
    #[inline(always)]
    pub fn is_pllp(&self) -> bool {
        *self == PLLPDIV_A::Pllp
    }
    #[doc = "Checks if the value of the field is `Div2`"]
    #[inline(always)]
    pub fn is_div2(&self) -> bool {
        *self == PLLPDIV_A::Div2
    }
    #[doc = "Checks if the value of the field is `Div3`"]
    #[inline(always)]
    pub fn is_div3(&self) -> bool {
        *self == PLLPDIV_A::Div3
    }
    #[doc = "Checks if the value of the field is `Div4`"]
    #[inline(always)]
    pub fn is_div4(&self) -> bool {
        *self == PLLPDIV_A::Div4
    }
    #[doc = "Checks if the value of the field is `Div5`"]
    #[inline(always)]
    pub fn is_div5(&self) -> bool {
        *self == PLLPDIV_A::Div5
    }
    #[doc = "Checks if the value of the field is `Div6`"]
    #[inline(always)]
    pub fn is_div6(&self) -> bool {
        *self == PLLPDIV_A::Div6
    }
    #[doc = "Checks if the value of the field is `Div7`"]
    #[inline(always)]
    pub fn is_div7(&self) -> bool {
        *self == PLLPDIV_A::Div7
    }
    #[doc = "Checks if the value of the field is `Div8`"]
    #[inline(always)]
    pub fn is_div8(&self) -> bool {
        *self == PLLPDIV_A::Div8
    }
    #[doc = "Checks if the value of the field is `Div9`"]
    #[inline(always)]
    pub fn is_div9(&self) -> bool {
        *self == PLLPDIV_A::Div9
    }
    #[doc = "Checks if the value of the field is `Div10`"]
    #[inline(always)]
    pub fn is_div10(&self) -> bool {
        *self == PLLPDIV_A::Div10
    }
    #[doc = "Checks if the value of the field is `Div11`"]
    #[inline(always)]
    pub fn is_div11(&self) -> bool {
        *self == PLLPDIV_A::Div11
    }
    #[doc = "Checks if the value of the field is `Div12`"]
    #[inline(always)]
    pub fn is_div12(&self) -> bool {
        *self == PLLPDIV_A::Div12
    }
    #[doc = "Checks if the value of the field is `Div13`"]
    #[inline(always)]
    pub fn is_div13(&self) -> bool {
        *self == PLLPDIV_A::Div13
    }
    #[doc = "Checks if the value of the field is `Div14`"]
    #[inline(always)]
    pub fn is_div14(&self) -> bool {
        *self == PLLPDIV_A::Div14
    }
    #[doc = "Checks if the value of the field is `Div15`"]
    #[inline(always)]
    pub fn is_div15(&self) -> bool {
        *self == PLLPDIV_A::Div15
    }
    #[doc = "Checks if the value of the field is `Div16`"]
    #[inline(always)]
    pub fn is_div16(&self) -> bool {
        *self == PLLPDIV_A::Div16
    }
    #[doc = "Checks if the value of the field is `Div17`"]
    #[inline(always)]
    pub fn is_div17(&self) -> bool {
        *self == PLLPDIV_A::Div17
    }
    #[doc = "Checks if the value of the field is `Div18`"]
    #[inline(always)]
    pub fn is_div18(&self) -> bool {
        *self == PLLPDIV_A::Div18
    }
    #[doc = "Checks if the value of the field is `Div19`"]
    #[inline(always)]
    pub fn is_div19(&self) -> bool {
        *self == PLLPDIV_A::Div19
    }
    #[doc = "Checks if the value of the field is `Div20`"]
    #[inline(always)]
    pub fn is_div20(&self) -> bool {
        *self == PLLPDIV_A::Div20
    }
    #[doc = "Checks if the value of the field is `Div21`"]
    #[inline(always)]
    pub fn is_div21(&self) -> bool {
        *self == PLLPDIV_A::Div21
    }
    #[doc = "Checks if the value of the field is `Div22`"]
    #[inline(always)]
    pub fn is_div22(&self) -> bool {
        *self == PLLPDIV_A::Div22
    }
    #[doc = "Checks if the value of the field is `Div23`"]
    #[inline(always)]
    pub fn is_div23(&self) -> bool {
        *self == PLLPDIV_A::Div23
    }
    #[doc = "Checks if the value of the field is `Div24`"]
    #[inline(always)]
    pub fn is_div24(&self) -> bool {
        *self == PLLPDIV_A::Div24
    }
    #[doc = "Checks if the value of the field is `Div25`"]
    #[inline(always)]
    pub fn is_div25(&self) -> bool {
        *self == PLLPDIV_A::Div25
    }
    #[doc = "Checks if the value of the field is `Div26`"]
    #[inline(always)]
    pub fn is_div26(&self) -> bool {
        *self == PLLPDIV_A::Div26
    }
    #[doc = "Checks if the value of the field is `Div27`"]
    #[inline(always)]
    pub fn is_div27(&self) -> bool {
        *self == PLLPDIV_A::Div27
    }
    #[doc = "Checks if the value of the field is `Div28`"]
    #[inline(always)]
    pub fn is_div28(&self) -> bool {
        *self == PLLPDIV_A::Div28
    }
    #[doc = "Checks if the value of the field is `Div29`"]
    #[inline(always)]
    pub fn is_div29(&self) -> bool {
        *self == PLLPDIV_A::Div29
    }
    #[doc = "Checks if the value of the field is `Div30`"]
    #[inline(always)]
    pub fn is_div30(&self) -> bool {
        *self == PLLPDIV_A::Div30
    }
    #[doc = "Checks if the value of the field is `Div31`"]
    #[inline(always)]
    pub fn is_div31(&self) -> bool {
        *self == PLLPDIV_A::Div31
    }
}
#[doc = "Field `PLLPDIV` writer - Main PLL division factor for PLLSAI2CLK"]
pub type PLLPDIV_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, PLLCFGR_SPEC, u8, PLLPDIV_A, 5, O>;
impl<'a, const O: u8> PLLPDIV_W<'a, O> {
    #[doc = "pll_p_ck is controlled by PLLP"]
    #[inline(always)]
    pub fn pllp(self) -> &'a mut W {
        self.variant(PLLPDIV_A::Pllp)
    }
    #[doc = "pll_p_ck = vco_ck / 2"]
    #[inline(always)]
    pub fn div2(self) -> &'a mut W {
        self.variant(PLLPDIV_A::Div2)
    }
    #[doc = "pll_p_ck = vco_ck / 3"]
    #[inline(always)]
    pub fn div3(self) -> &'a mut W {
        self.variant(PLLPDIV_A::Div3)
    }
    #[doc = "pll_p_ck = vco_ck / 4"]
    #[inline(always)]
    pub fn div4(self) -> &'a mut W {
        self.variant(PLLPDIV_A::Div4)
    }
    #[doc = "pll_p_ck = vco_ck / 5"]
    #[inline(always)]
    pub fn div5(self) -> &'a mut W {
        self.variant(PLLPDIV_A::Div5)
    }
    #[doc = "pll_p_ck = vco_ck / 6"]
    #[inline(always)]
    pub fn div6(self) -> &'a mut W {
        self.variant(PLLPDIV_A::Div6)
    }
    #[doc = "pll_p_ck = vco_ck / 7"]
    #[inline(always)]
    pub fn div7(self) -> &'a mut W {
        self.variant(PLLPDIV_A::Div7)
    }
    #[doc = "pll_p_ck = vco_ck / 8"]
    #[inline(always)]
    pub fn div8(self) -> &'a mut W {
        self.variant(PLLPDIV_A::Div8)
    }
    #[doc = "pll_p_ck = vco_ck / 9"]
    #[inline(always)]
    pub fn div9(self) -> &'a mut W {
        self.variant(PLLPDIV_A::Div9)
    }
    #[doc = "pll_p_ck = vco_ck / 10"]
    #[inline(always)]
    pub fn div10(self) -> &'a mut W {
        self.variant(PLLPDIV_A::Div10)
    }
    #[doc = "pll_p_ck = vco_ck / 11"]
    #[inline(always)]
    pub fn div11(self) -> &'a mut W {
        self.variant(PLLPDIV_A::Div11)
    }
    #[doc = "pll_p_ck = vco_ck / 12"]
    #[inline(always)]
    pub fn div12(self) -> &'a mut W {
        self.variant(PLLPDIV_A::Div12)
    }
    #[doc = "pll_p_ck = vco_ck / 13"]
    #[inline(always)]
    pub fn div13(self) -> &'a mut W {
        self.variant(PLLPDIV_A::Div13)
    }
    #[doc = "pll_p_ck = vco_ck / 14"]
    #[inline(always)]
    pub fn div14(self) -> &'a mut W {
        self.variant(PLLPDIV_A::Div14)
    }
    #[doc = "pll_p_ck = vco_ck / 15"]
    #[inline(always)]
    pub fn div15(self) -> &'a mut W {
        self.variant(PLLPDIV_A::Div15)
    }
    #[doc = "pll_p_ck = vco_ck / 16"]
    #[inline(always)]
    pub fn div16(self) -> &'a mut W {
        self.variant(PLLPDIV_A::Div16)
    }
    #[doc = "pll_p_ck = vco_ck / 17"]
    #[inline(always)]
    pub fn div17(self) -> &'a mut W {
        self.variant(PLLPDIV_A::Div17)
    }
    #[doc = "pll_p_ck = vco_ck / 18"]
    #[inline(always)]
    pub fn div18(self) -> &'a mut W {
        self.variant(PLLPDIV_A::Div18)
    }
    #[doc = "pll_p_ck = vco_ck / 19"]
    #[inline(always)]
    pub fn div19(self) -> &'a mut W {
        self.variant(PLLPDIV_A::Div19)
    }
    #[doc = "pll_p_ck = vco_ck / 20"]
    #[inline(always)]
    pub fn div20(self) -> &'a mut W {
        self.variant(PLLPDIV_A::Div20)
    }
    #[doc = "pll_p_ck = vco_ck / 21"]
    #[inline(always)]
    pub fn div21(self) -> &'a mut W {
        self.variant(PLLPDIV_A::Div21)
    }
    #[doc = "pll_p_ck = vco_ck / 22"]
    #[inline(always)]
    pub fn div22(self) -> &'a mut W {
        self.variant(PLLPDIV_A::Div22)
    }
    #[doc = "pll_p_ck = vco_ck / 23"]
    #[inline(always)]
    pub fn div23(self) -> &'a mut W {
        self.variant(PLLPDIV_A::Div23)
    }
    #[doc = "pll_p_ck = vco_ck / 24"]
    #[inline(always)]
    pub fn div24(self) -> &'a mut W {
        self.variant(PLLPDIV_A::Div24)
    }
    #[doc = "pll_p_ck = vco_ck / 25"]
    #[inline(always)]
    pub fn div25(self) -> &'a mut W {
        self.variant(PLLPDIV_A::Div25)
    }
    #[doc = "pll_p_ck = vco_ck / 26"]
    #[inline(always)]
    pub fn div26(self) -> &'a mut W {
        self.variant(PLLPDIV_A::Div26)
    }
    #[doc = "pll_p_ck = vco_ck / 27"]
    #[inline(always)]
    pub fn div27(self) -> &'a mut W {
        self.variant(PLLPDIV_A::Div27)
    }
    #[doc = "pll_p_ck = vco_ck / 28"]
    #[inline(always)]
    pub fn div28(self) -> &'a mut W {
        self.variant(PLLPDIV_A::Div28)
    }
    #[doc = "pll_p_ck = vco_ck / 29"]
    #[inline(always)]
    pub fn div29(self) -> &'a mut W {
        self.variant(PLLPDIV_A::Div29)
    }
    #[doc = "pll_p_ck = vco_ck / 30"]
    #[inline(always)]
    pub fn div30(self) -> &'a mut W {
        self.variant(PLLPDIV_A::Div30)
    }
    #[doc = "pll_p_ck = vco_ck / 31"]
    #[inline(always)]
    pub fn div31(self) -> &'a mut W {
        self.variant(PLLPDIV_A::Div31)
    }
}
impl R {
    #[doc = "Bits 0:1 - Main PLL, PLLSAI1 and PLLSAI2 entry clock source"]
    #[inline(always)]
    pub fn pllsrc(&self) -> PLLSRC_R {
        PLLSRC_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 4:7 - Division factor for the main PLL and audio PLL (PLLSAI1 and PLLSAI2) input clock"]
    #[inline(always)]
    pub fn pllm(&self) -> PLLM_R {
        PLLM_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:14 - Main PLL multiplication factor for VCO"]
    #[inline(always)]
    pub fn plln(&self) -> PLLN_R {
        PLLN_R::new(((self.bits >> 8) & 0x7f) as u8)
    }
    #[doc = "Bit 16 - Main PLL PLLSAI3CLK output enable"]
    #[inline(always)]
    pub fn pllpen(&self) -> PLLPEN_R {
        PLLPEN_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Main PLL division factor for PLLSAI3CLK (SAI1 and SAI2 clock)"]
    #[inline(always)]
    pub fn pllp(&self) -> PLLP_R {
        PLLP_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 20 - Main PLL PLLUSB1CLK output enable"]
    #[inline(always)]
    pub fn pllqen(&self) -> PLLQEN_R {
        PLLQEN_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bits 21:22 - Main PLL division factor for PLLUSB1CLK(48 MHz clock)"]
    #[inline(always)]
    pub fn pllq(&self) -> PLLQ_R {
        PLLQ_R::new(((self.bits >> 21) & 3) as u8)
    }
    #[doc = "Bit 24 - Main PLL PLLCLK output enable"]
    #[inline(always)]
    pub fn pllren(&self) -> PLLREN_R {
        PLLREN_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bits 25:26 - Main PLL division factor for PLLCLK (system clock)"]
    #[inline(always)]
    pub fn pllr(&self) -> PLLR_R {
        PLLR_R::new(((self.bits >> 25) & 3) as u8)
    }
    #[doc = "Bits 27:31 - Main PLL division factor for PLLSAI2CLK"]
    #[inline(always)]
    pub fn pllpdiv(&self) -> PLLPDIV_R {
        PLLPDIV_R::new(((self.bits >> 27) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Main PLL, PLLSAI1 and PLLSAI2 entry clock source"]
    #[inline(always)]
    #[must_use]
    pub fn pllsrc(&mut self) -> PLLSRC_W<0> {
        PLLSRC_W::new(self)
    }
    #[doc = "Bits 4:7 - Division factor for the main PLL and audio PLL (PLLSAI1 and PLLSAI2) input clock"]
    #[inline(always)]
    #[must_use]
    pub fn pllm(&mut self) -> PLLM_W<4> {
        PLLM_W::new(self)
    }
    #[doc = "Bits 8:14 - Main PLL multiplication factor for VCO"]
    #[inline(always)]
    #[must_use]
    pub fn plln(&mut self) -> PLLN_W<8> {
        PLLN_W::new(self)
    }
    #[doc = "Bit 16 - Main PLL PLLSAI3CLK output enable"]
    #[inline(always)]
    #[must_use]
    pub fn pllpen(&mut self) -> PLLPEN_W<16> {
        PLLPEN_W::new(self)
    }
    #[doc = "Bit 17 - Main PLL division factor for PLLSAI3CLK (SAI1 and SAI2 clock)"]
    #[inline(always)]
    #[must_use]
    pub fn pllp(&mut self) -> PLLP_W<17> {
        PLLP_W::new(self)
    }
    #[doc = "Bit 20 - Main PLL PLLUSB1CLK output enable"]
    #[inline(always)]
    #[must_use]
    pub fn pllqen(&mut self) -> PLLQEN_W<20> {
        PLLQEN_W::new(self)
    }
    #[doc = "Bits 21:22 - Main PLL division factor for PLLUSB1CLK(48 MHz clock)"]
    #[inline(always)]
    #[must_use]
    pub fn pllq(&mut self) -> PLLQ_W<21> {
        PLLQ_W::new(self)
    }
    #[doc = "Bit 24 - Main PLL PLLCLK output enable"]
    #[inline(always)]
    #[must_use]
    pub fn pllren(&mut self) -> PLLREN_W<24> {
        PLLREN_W::new(self)
    }
    #[doc = "Bits 25:26 - Main PLL division factor for PLLCLK (system clock)"]
    #[inline(always)]
    #[must_use]
    pub fn pllr(&mut self) -> PLLR_W<25> {
        PLLR_W::new(self)
    }
    #[doc = "Bits 27:31 - Main PLL division factor for PLLSAI2CLK"]
    #[inline(always)]
    #[must_use]
    pub fn pllpdiv(&mut self) -> PLLPDIV_W<27> {
        PLLPDIV_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "PLL configuration register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pllcfgr](index.html) module"]
pub struct PLLCFGR_SPEC;
impl crate::RegisterSpec for PLLCFGR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pllcfgr::R](R) reader structure"]
impl crate::Readable for PLLCFGR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pllcfgr::W](W) writer structure"]
impl crate::Writable for PLLCFGR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PLLCFGR to value 0x1000"]
impl crate::Resettable for PLLCFGR_SPEC {
    const RESET_VALUE: Self::Ux = 0x1000;
}
