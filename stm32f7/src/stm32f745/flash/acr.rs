#[doc = "Register `ACR` reader"]
pub struct R(crate::R<ACR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ACR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ACR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ACR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ACR` writer"]
pub struct W(crate::W<ACR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ACR_SPEC>;
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
impl From<crate::W<ACR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ACR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `LATENCY` reader - Latency"]
pub type LATENCY_R = crate::FieldReader<u8, LATENCY_A>;
#[doc = "Latency\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum LATENCY_A {
    #[doc = "0: 0 wait states"]
    Ws0 = 0,
    #[doc = "1: 1 wait states"]
    Ws1 = 1,
    #[doc = "2: 2 wait states"]
    Ws2 = 2,
    #[doc = "3: 3 wait states"]
    Ws3 = 3,
    #[doc = "4: 4 wait states"]
    Ws4 = 4,
    #[doc = "5: 5 wait states"]
    Ws5 = 5,
    #[doc = "6: 6 wait states"]
    Ws6 = 6,
    #[doc = "7: 7 wait states"]
    Ws7 = 7,
    #[doc = "8: 8 wait states"]
    Ws8 = 8,
    #[doc = "9: 9 wait states"]
    Ws9 = 9,
    #[doc = "10: 10 wait states"]
    Ws10 = 10,
    #[doc = "11: 11 wait states"]
    Ws11 = 11,
    #[doc = "12: 12 wait states"]
    Ws12 = 12,
    #[doc = "13: 13 wait states"]
    Ws13 = 13,
    #[doc = "14: 14 wait states"]
    Ws14 = 14,
    #[doc = "15: 15 wait states"]
    Ws15 = 15,
}
impl From<LATENCY_A> for u8 {
    #[inline(always)]
    fn from(variant: LATENCY_A) -> Self {
        variant as _
    }
}
impl LATENCY_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LATENCY_A {
        match self.bits {
            0 => LATENCY_A::Ws0,
            1 => LATENCY_A::Ws1,
            2 => LATENCY_A::Ws2,
            3 => LATENCY_A::Ws3,
            4 => LATENCY_A::Ws4,
            5 => LATENCY_A::Ws5,
            6 => LATENCY_A::Ws6,
            7 => LATENCY_A::Ws7,
            8 => LATENCY_A::Ws8,
            9 => LATENCY_A::Ws9,
            10 => LATENCY_A::Ws10,
            11 => LATENCY_A::Ws11,
            12 => LATENCY_A::Ws12,
            13 => LATENCY_A::Ws13,
            14 => LATENCY_A::Ws14,
            15 => LATENCY_A::Ws15,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `Ws0`"]
    #[inline(always)]
    pub fn is_ws0(&self) -> bool {
        *self == LATENCY_A::Ws0
    }
    #[doc = "Checks if the value of the field is `Ws1`"]
    #[inline(always)]
    pub fn is_ws1(&self) -> bool {
        *self == LATENCY_A::Ws1
    }
    #[doc = "Checks if the value of the field is `Ws2`"]
    #[inline(always)]
    pub fn is_ws2(&self) -> bool {
        *self == LATENCY_A::Ws2
    }
    #[doc = "Checks if the value of the field is `Ws3`"]
    #[inline(always)]
    pub fn is_ws3(&self) -> bool {
        *self == LATENCY_A::Ws3
    }
    #[doc = "Checks if the value of the field is `Ws4`"]
    #[inline(always)]
    pub fn is_ws4(&self) -> bool {
        *self == LATENCY_A::Ws4
    }
    #[doc = "Checks if the value of the field is `Ws5`"]
    #[inline(always)]
    pub fn is_ws5(&self) -> bool {
        *self == LATENCY_A::Ws5
    }
    #[doc = "Checks if the value of the field is `Ws6`"]
    #[inline(always)]
    pub fn is_ws6(&self) -> bool {
        *self == LATENCY_A::Ws6
    }
    #[doc = "Checks if the value of the field is `Ws7`"]
    #[inline(always)]
    pub fn is_ws7(&self) -> bool {
        *self == LATENCY_A::Ws7
    }
    #[doc = "Checks if the value of the field is `Ws8`"]
    #[inline(always)]
    pub fn is_ws8(&self) -> bool {
        *self == LATENCY_A::Ws8
    }
    #[doc = "Checks if the value of the field is `Ws9`"]
    #[inline(always)]
    pub fn is_ws9(&self) -> bool {
        *self == LATENCY_A::Ws9
    }
    #[doc = "Checks if the value of the field is `Ws10`"]
    #[inline(always)]
    pub fn is_ws10(&self) -> bool {
        *self == LATENCY_A::Ws10
    }
    #[doc = "Checks if the value of the field is `Ws11`"]
    #[inline(always)]
    pub fn is_ws11(&self) -> bool {
        *self == LATENCY_A::Ws11
    }
    #[doc = "Checks if the value of the field is `Ws12`"]
    #[inline(always)]
    pub fn is_ws12(&self) -> bool {
        *self == LATENCY_A::Ws12
    }
    #[doc = "Checks if the value of the field is `Ws13`"]
    #[inline(always)]
    pub fn is_ws13(&self) -> bool {
        *self == LATENCY_A::Ws13
    }
    #[doc = "Checks if the value of the field is `Ws14`"]
    #[inline(always)]
    pub fn is_ws14(&self) -> bool {
        *self == LATENCY_A::Ws14
    }
    #[doc = "Checks if the value of the field is `Ws15`"]
    #[inline(always)]
    pub fn is_ws15(&self) -> bool {
        *self == LATENCY_A::Ws15
    }
}
#[doc = "Field `LATENCY` writer - Latency"]
pub type LATENCY_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, ACR_SPEC, u8, LATENCY_A, 4, O>;
impl<'a, const O: u8> LATENCY_W<'a, O> {
    #[doc = "0 wait states"]
    #[inline(always)]
    pub fn ws0(self) -> &'a mut W {
        self.variant(LATENCY_A::Ws0)
    }
    #[doc = "1 wait states"]
    #[inline(always)]
    pub fn ws1(self) -> &'a mut W {
        self.variant(LATENCY_A::Ws1)
    }
    #[doc = "2 wait states"]
    #[inline(always)]
    pub fn ws2(self) -> &'a mut W {
        self.variant(LATENCY_A::Ws2)
    }
    #[doc = "3 wait states"]
    #[inline(always)]
    pub fn ws3(self) -> &'a mut W {
        self.variant(LATENCY_A::Ws3)
    }
    #[doc = "4 wait states"]
    #[inline(always)]
    pub fn ws4(self) -> &'a mut W {
        self.variant(LATENCY_A::Ws4)
    }
    #[doc = "5 wait states"]
    #[inline(always)]
    pub fn ws5(self) -> &'a mut W {
        self.variant(LATENCY_A::Ws5)
    }
    #[doc = "6 wait states"]
    #[inline(always)]
    pub fn ws6(self) -> &'a mut W {
        self.variant(LATENCY_A::Ws6)
    }
    #[doc = "7 wait states"]
    #[inline(always)]
    pub fn ws7(self) -> &'a mut W {
        self.variant(LATENCY_A::Ws7)
    }
    #[doc = "8 wait states"]
    #[inline(always)]
    pub fn ws8(self) -> &'a mut W {
        self.variant(LATENCY_A::Ws8)
    }
    #[doc = "9 wait states"]
    #[inline(always)]
    pub fn ws9(self) -> &'a mut W {
        self.variant(LATENCY_A::Ws9)
    }
    #[doc = "10 wait states"]
    #[inline(always)]
    pub fn ws10(self) -> &'a mut W {
        self.variant(LATENCY_A::Ws10)
    }
    #[doc = "11 wait states"]
    #[inline(always)]
    pub fn ws11(self) -> &'a mut W {
        self.variant(LATENCY_A::Ws11)
    }
    #[doc = "12 wait states"]
    #[inline(always)]
    pub fn ws12(self) -> &'a mut W {
        self.variant(LATENCY_A::Ws12)
    }
    #[doc = "13 wait states"]
    #[inline(always)]
    pub fn ws13(self) -> &'a mut W {
        self.variant(LATENCY_A::Ws13)
    }
    #[doc = "14 wait states"]
    #[inline(always)]
    pub fn ws14(self) -> &'a mut W {
        self.variant(LATENCY_A::Ws14)
    }
    #[doc = "15 wait states"]
    #[inline(always)]
    pub fn ws15(self) -> &'a mut W {
        self.variant(LATENCY_A::Ws15)
    }
}
#[doc = "Field `PRFTEN` reader - Prefetch enable"]
pub type PRFTEN_R = crate::BitReader<PRFTEN_A>;
#[doc = "Prefetch enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PRFTEN_A {
    #[doc = "0: Prefetch is disabled"]
    Disabled = 0,
    #[doc = "1: Prefetch is enabled"]
    Enabled = 1,
}
impl From<PRFTEN_A> for bool {
    #[inline(always)]
    fn from(variant: PRFTEN_A) -> Self {
        variant as u8 != 0
    }
}
impl PRFTEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PRFTEN_A {
        match self.bits {
            false => PRFTEN_A::Disabled,
            true => PRFTEN_A::Enabled,
        }
    }
    #[doc = "Checks if the value of the field is `Disabled`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == PRFTEN_A::Disabled
    }
    #[doc = "Checks if the value of the field is `Enabled`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == PRFTEN_A::Enabled
    }
}
#[doc = "Field `PRFTEN` writer - Prefetch enable"]
pub type PRFTEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, ACR_SPEC, PRFTEN_A, O>;
impl<'a, const O: u8> PRFTEN_W<'a, O> {
    #[doc = "Prefetch is disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(PRFTEN_A::Disabled)
    }
    #[doc = "Prefetch is enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(PRFTEN_A::Enabled)
    }
}
#[doc = "Field `ARTEN` reader - ART Accelerator Enable"]
pub type ARTEN_R = crate::BitReader<ARTEN_A>;
#[doc = "ART Accelerator Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ARTEN_A {
    #[doc = "0: ART Accelerator is disabled"]
    Disabled = 0,
    #[doc = "1: ART Accelerator is enabled"]
    Enabled = 1,
}
impl From<ARTEN_A> for bool {
    #[inline(always)]
    fn from(variant: ARTEN_A) -> Self {
        variant as u8 != 0
    }
}
impl ARTEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ARTEN_A {
        match self.bits {
            false => ARTEN_A::Disabled,
            true => ARTEN_A::Enabled,
        }
    }
    #[doc = "Checks if the value of the field is `Disabled`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == ARTEN_A::Disabled
    }
    #[doc = "Checks if the value of the field is `Enabled`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == ARTEN_A::Enabled
    }
}
#[doc = "Field `ARTEN` writer - ART Accelerator Enable"]
pub type ARTEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, ACR_SPEC, ARTEN_A, O>;
impl<'a, const O: u8> ARTEN_W<'a, O> {
    #[doc = "ART Accelerator is disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(ARTEN_A::Disabled)
    }
    #[doc = "ART Accelerator is enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(ARTEN_A::Enabled)
    }
}
#[doc = "Field `ARTRST` reader - ART Accelerator reset"]
pub type ARTRST_R = crate::BitReader<ARTRST_A>;
#[doc = "ART Accelerator reset\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ARTRST_A {
    #[doc = "0: Accelerator is not reset"]
    NotReset = 0,
    #[doc = "1: Accelerator is reset"]
    Reset = 1,
}
impl From<ARTRST_A> for bool {
    #[inline(always)]
    fn from(variant: ARTRST_A) -> Self {
        variant as u8 != 0
    }
}
impl ARTRST_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ARTRST_A {
        match self.bits {
            false => ARTRST_A::NotReset,
            true => ARTRST_A::Reset,
        }
    }
    #[doc = "Checks if the value of the field is `NotReset`"]
    #[inline(always)]
    pub fn is_not_reset(&self) -> bool {
        *self == ARTRST_A::NotReset
    }
    #[doc = "Checks if the value of the field is `Reset`"]
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        *self == ARTRST_A::Reset
    }
}
#[doc = "Field `ARTRST` writer - ART Accelerator reset"]
pub type ARTRST_W<'a, const O: u8> = crate::BitWriter<'a, u32, ACR_SPEC, ARTRST_A, O>;
impl<'a, const O: u8> ARTRST_W<'a, O> {
    #[doc = "Accelerator is not reset"]
    #[inline(always)]
    pub fn not_reset(self) -> &'a mut W {
        self.variant(ARTRST_A::NotReset)
    }
    #[doc = "Accelerator is reset"]
    #[inline(always)]
    pub fn reset(self) -> &'a mut W {
        self.variant(ARTRST_A::Reset)
    }
}
impl R {
    #[doc = "Bits 0:3 - Latency"]
    #[inline(always)]
    pub fn latency(&self) -> LATENCY_R {
        LATENCY_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bit 8 - Prefetch enable"]
    #[inline(always)]
    pub fn prften(&self) -> PRFTEN_R {
        PRFTEN_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - ART Accelerator Enable"]
    #[inline(always)]
    pub fn arten(&self) -> ARTEN_R {
        ARTEN_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 11 - ART Accelerator reset"]
    #[inline(always)]
    pub fn artrst(&self) -> ARTRST_R {
        ARTRST_R::new(((self.bits >> 11) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3 - Latency"]
    #[inline(always)]
    #[must_use]
    pub fn latency(&mut self) -> LATENCY_W<0> {
        LATENCY_W::new(self)
    }
    #[doc = "Bit 8 - Prefetch enable"]
    #[inline(always)]
    #[must_use]
    pub fn prften(&mut self) -> PRFTEN_W<8> {
        PRFTEN_W::new(self)
    }
    #[doc = "Bit 9 - ART Accelerator Enable"]
    #[inline(always)]
    #[must_use]
    pub fn arten(&mut self) -> ARTEN_W<9> {
        ARTEN_W::new(self)
    }
    #[doc = "Bit 11 - ART Accelerator reset"]
    #[inline(always)]
    #[must_use]
    pub fn artrst(&mut self) -> ARTRST_W<11> {
        ARTRST_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Flash access control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [acr](index.html) module"]
pub struct ACR_SPEC;
impl crate::RegisterSpec for ACR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [acr::R](R) reader structure"]
impl crate::Readable for ACR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [acr::W](W) writer structure"]
impl crate::Writable for ACR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ACR to value 0"]
impl crate::Resettable for ACR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
