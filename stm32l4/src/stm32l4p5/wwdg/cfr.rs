#[doc = "Register `CFR` reader"]
pub struct R(crate::R<CFR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CFR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CFR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CFR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CFR` writer"]
pub struct W(crate::W<CFR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CFR_SPEC>;
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
impl From<crate::W<CFR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CFR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `W` reader - 7-bit window value"]
pub type W_R = crate::FieldReader<u8, u8>;
#[doc = "Field `W` writer - 7-bit window value"]
pub type W_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, CFR_SPEC, u8, u8, 7, O>;
#[doc = "Field `WDGTB` reader - Timer base"]
pub type WDGTB_R = crate::FieldReader<u8, WDGTB_A>;
#[doc = "Timer base\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum WDGTB_A {
    #[doc = "0: Counter clock (PCLK1 div 4096) div 1"]
    Div1 = 0,
    #[doc = "1: Counter clock (PCLK1 div 4096) div 2"]
    Div2 = 1,
    #[doc = "2: Counter clock (PCLK1 div 4096) div 4"]
    Div4 = 2,
    #[doc = "3: Counter clock (PCLK1 div 4096) div 8"]
    Div8 = 3,
}
impl From<WDGTB_A> for u8 {
    #[inline(always)]
    fn from(variant: WDGTB_A) -> Self {
        variant as _
    }
}
impl WDGTB_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WDGTB_A {
        match self.bits {
            0 => WDGTB_A::Div1,
            1 => WDGTB_A::Div2,
            2 => WDGTB_A::Div4,
            3 => WDGTB_A::Div8,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `Div1`"]
    #[inline(always)]
    pub fn is_div1(&self) -> bool {
        *self == WDGTB_A::Div1
    }
    #[doc = "Checks if the value of the field is `Div2`"]
    #[inline(always)]
    pub fn is_div2(&self) -> bool {
        *self == WDGTB_A::Div2
    }
    #[doc = "Checks if the value of the field is `Div4`"]
    #[inline(always)]
    pub fn is_div4(&self) -> bool {
        *self == WDGTB_A::Div4
    }
    #[doc = "Checks if the value of the field is `Div8`"]
    #[inline(always)]
    pub fn is_div8(&self) -> bool {
        *self == WDGTB_A::Div8
    }
}
#[doc = "Field `WDGTB` writer - Timer base"]
pub type WDGTB_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, CFR_SPEC, u8, WDGTB_A, 2, O>;
impl<'a, const O: u8> WDGTB_W<'a, O> {
    #[doc = "Counter clock (PCLK1 div 4096) div 1"]
    #[inline(always)]
    pub fn div1(self) -> &'a mut W {
        self.variant(WDGTB_A::Div1)
    }
    #[doc = "Counter clock (PCLK1 div 4096) div 2"]
    #[inline(always)]
    pub fn div2(self) -> &'a mut W {
        self.variant(WDGTB_A::Div2)
    }
    #[doc = "Counter clock (PCLK1 div 4096) div 4"]
    #[inline(always)]
    pub fn div4(self) -> &'a mut W {
        self.variant(WDGTB_A::Div4)
    }
    #[doc = "Counter clock (PCLK1 div 4096) div 8"]
    #[inline(always)]
    pub fn div8(self) -> &'a mut W {
        self.variant(WDGTB_A::Div8)
    }
}
#[doc = "Field `EWI` reader - Early wakeup interrupt"]
pub type EWI_R = crate::BitReader<EWIW_A>;
#[doc = "Early wakeup interrupt\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EWIW_A {
    #[doc = "1: interrupt occurs whenever the counter reaches the value 0x40"]
    Enable = 1,
}
impl From<EWIW_A> for bool {
    #[inline(always)]
    fn from(variant: EWIW_A) -> Self {
        variant as u8 != 0
    }
}
impl EWI_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<EWIW_A> {
        match self.bits {
            true => Some(EWIW_A::Enable),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `Enable`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == EWIW_A::Enable
    }
}
#[doc = "Field `EWI` writer - Early wakeup interrupt"]
pub type EWI_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFR_SPEC, EWIW_A, O>;
impl<'a, const O: u8> EWI_W<'a, O> {
    #[doc = "interrupt occurs whenever the counter reaches the value 0x40"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(EWIW_A::Enable)
    }
}
impl R {
    #[doc = "Bits 0:6 - 7-bit window value"]
    #[inline(always)]
    pub fn w(&self) -> W_R {
        W_R::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bits 7:8 - Timer base"]
    #[inline(always)]
    pub fn wdgtb(&self) -> WDGTB_R {
        WDGTB_R::new(((self.bits >> 7) & 3) as u8)
    }
    #[doc = "Bit 9 - Early wakeup interrupt"]
    #[inline(always)]
    pub fn ewi(&self) -> EWI_R {
        EWI_R::new(((self.bits >> 9) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:6 - 7-bit window value"]
    #[inline(always)]
    #[must_use]
    pub fn w(&mut self) -> W_W<0> {
        W_W::new(self)
    }
    #[doc = "Bits 7:8 - Timer base"]
    #[inline(always)]
    #[must_use]
    pub fn wdgtb(&mut self) -> WDGTB_W<7> {
        WDGTB_W::new(self)
    }
    #[doc = "Bit 9 - Early wakeup interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn ewi(&mut self) -> EWI_W<9> {
        EWI_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Configuration register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cfr](index.html) module"]
pub struct CFR_SPEC;
impl crate::RegisterSpec for CFR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cfr::R](R) reader structure"]
impl crate::Readable for CFR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cfr::W](W) writer structure"]
impl crate::Writable for CFR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CFR to value 0x7f"]
impl crate::Resettable for CFR_SPEC {
    const RESET_VALUE: Self::Ux = 0x7f;
}
