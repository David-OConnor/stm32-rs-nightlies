#[doc = "Register `CLRFR` reader"]
pub struct R(crate::R<CLRFR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CLRFR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CLRFR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CLRFR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CLRFR` writer"]
pub struct W(crate::W<CLRFR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CLRFR_SPEC>;
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
impl From<crate::W<CLRFR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CLRFR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `COVRUDR` reader - Clear overrun / underrun"]
pub type COVRUDR_R = crate::BitReader<COVRUDRW_A>;
#[doc = "Clear overrun / underrun\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum COVRUDRW_A {
    #[doc = "1: Clears the OVRUDR flag"]
    Clear = 1,
}
impl From<COVRUDRW_A> for bool {
    #[inline(always)]
    fn from(variant: COVRUDRW_A) -> Self {
        variant as u8 != 0
    }
}
impl COVRUDR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<COVRUDRW_A> {
        match self.bits {
            true => Some(COVRUDRW_A::Clear),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `Clear`"]
    #[inline(always)]
    pub fn is_clear(&self) -> bool {
        *self == COVRUDRW_A::Clear
    }
}
#[doc = "Field `COVRUDR` writer - Clear overrun / underrun"]
pub type COVRUDR_W<'a, const O: u8> = crate::BitWriter<'a, u32, CLRFR_SPEC, COVRUDRW_A, O>;
impl<'a, const O: u8> COVRUDR_W<'a, O> {
    #[doc = "Clears the OVRUDR flag"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(COVRUDRW_A::Clear)
    }
}
#[doc = "Field `CMUTEDET` reader - Mute detection flag"]
pub type CMUTEDET_R = crate::BitReader<CMUTEDETW_A>;
#[doc = "Mute detection flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CMUTEDETW_A {
    #[doc = "1: Clears the MUTEDET flag"]
    Clear = 1,
}
impl From<CMUTEDETW_A> for bool {
    #[inline(always)]
    fn from(variant: CMUTEDETW_A) -> Self {
        variant as u8 != 0
    }
}
impl CMUTEDET_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<CMUTEDETW_A> {
        match self.bits {
            true => Some(CMUTEDETW_A::Clear),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `Clear`"]
    #[inline(always)]
    pub fn is_clear(&self) -> bool {
        *self == CMUTEDETW_A::Clear
    }
}
#[doc = "Field `CMUTEDET` writer - Mute detection flag"]
pub type CMUTEDET_W<'a, const O: u8> = crate::BitWriter<'a, u32, CLRFR_SPEC, CMUTEDETW_A, O>;
impl<'a, const O: u8> CMUTEDET_W<'a, O> {
    #[doc = "Clears the MUTEDET flag"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(CMUTEDETW_A::Clear)
    }
}
#[doc = "Field `CWCKCFG` reader - Clear wrong clock configuration flag"]
pub type CWCKCFG_R = crate::BitReader<CWCKCFGW_A>;
#[doc = "Clear wrong clock configuration flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CWCKCFGW_A {
    #[doc = "1: Clears the WCKCFG flag"]
    Clear = 1,
}
impl From<CWCKCFGW_A> for bool {
    #[inline(always)]
    fn from(variant: CWCKCFGW_A) -> Self {
        variant as u8 != 0
    }
}
impl CWCKCFG_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<CWCKCFGW_A> {
        match self.bits {
            true => Some(CWCKCFGW_A::Clear),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `Clear`"]
    #[inline(always)]
    pub fn is_clear(&self) -> bool {
        *self == CWCKCFGW_A::Clear
    }
}
#[doc = "Field `CWCKCFG` writer - Clear wrong clock configuration flag"]
pub type CWCKCFG_W<'a, const O: u8> = crate::BitWriter<'a, u32, CLRFR_SPEC, CWCKCFGW_A, O>;
impl<'a, const O: u8> CWCKCFG_W<'a, O> {
    #[doc = "Clears the WCKCFG flag"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(CWCKCFGW_A::Clear)
    }
}
#[doc = "Field `CCNRDY` reader - Clear codec not ready flag"]
pub type CCNRDY_R = crate::BitReader<CCNRDYW_A>;
#[doc = "Clear codec not ready flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CCNRDYW_A {
    #[doc = "1: Clears the CNRDY flag"]
    Clear = 1,
}
impl From<CCNRDYW_A> for bool {
    #[inline(always)]
    fn from(variant: CCNRDYW_A) -> Self {
        variant as u8 != 0
    }
}
impl CCNRDY_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<CCNRDYW_A> {
        match self.bits {
            true => Some(CCNRDYW_A::Clear),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `Clear`"]
    #[inline(always)]
    pub fn is_clear(&self) -> bool {
        *self == CCNRDYW_A::Clear
    }
}
#[doc = "Field `CCNRDY` writer - Clear codec not ready flag"]
pub type CCNRDY_W<'a, const O: u8> = crate::BitWriter<'a, u32, CLRFR_SPEC, CCNRDYW_A, O>;
impl<'a, const O: u8> CCNRDY_W<'a, O> {
    #[doc = "Clears the CNRDY flag"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(CCNRDYW_A::Clear)
    }
}
#[doc = "Field `CAFSDET` reader - Clear anticipated frame synchronization detection flag"]
pub type CAFSDET_R = crate::BitReader<CAFSDETW_A>;
#[doc = "Clear anticipated frame synchronization detection flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CAFSDETW_A {
    #[doc = "1: Clears the AFSDET flag"]
    Clear = 1,
}
impl From<CAFSDETW_A> for bool {
    #[inline(always)]
    fn from(variant: CAFSDETW_A) -> Self {
        variant as u8 != 0
    }
}
impl CAFSDET_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<CAFSDETW_A> {
        match self.bits {
            true => Some(CAFSDETW_A::Clear),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `Clear`"]
    #[inline(always)]
    pub fn is_clear(&self) -> bool {
        *self == CAFSDETW_A::Clear
    }
}
#[doc = "Field `CAFSDET` writer - Clear anticipated frame synchronization detection flag"]
pub type CAFSDET_W<'a, const O: u8> = crate::BitWriter<'a, u32, CLRFR_SPEC, CAFSDETW_A, O>;
impl<'a, const O: u8> CAFSDET_W<'a, O> {
    #[doc = "Clears the AFSDET flag"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(CAFSDETW_A::Clear)
    }
}
#[doc = "Field `CLFSDET` reader - Clear late frame synchronization detection flag"]
pub type CLFSDET_R = crate::BitReader<CLFSDETW_A>;
#[doc = "Clear late frame synchronization detection flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CLFSDETW_A {
    #[doc = "1: Clears the LFSDET flag"]
    Clear = 1,
}
impl From<CLFSDETW_A> for bool {
    #[inline(always)]
    fn from(variant: CLFSDETW_A) -> Self {
        variant as u8 != 0
    }
}
impl CLFSDET_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<CLFSDETW_A> {
        match self.bits {
            true => Some(CLFSDETW_A::Clear),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `Clear`"]
    #[inline(always)]
    pub fn is_clear(&self) -> bool {
        *self == CLFSDETW_A::Clear
    }
}
#[doc = "Field `CLFSDET` writer - Clear late frame synchronization detection flag"]
pub type CLFSDET_W<'a, const O: u8> = crate::BitWriter<'a, u32, CLRFR_SPEC, CLFSDETW_A, O>;
impl<'a, const O: u8> CLFSDET_W<'a, O> {
    #[doc = "Clears the LFSDET flag"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(CLFSDETW_A::Clear)
    }
}
impl R {
    #[doc = "Bit 0 - Clear overrun / underrun"]
    #[inline(always)]
    pub fn covrudr(&self) -> COVRUDR_R {
        COVRUDR_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Mute detection flag"]
    #[inline(always)]
    pub fn cmutedet(&self) -> CMUTEDET_R {
        CMUTEDET_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Clear wrong clock configuration flag"]
    #[inline(always)]
    pub fn cwckcfg(&self) -> CWCKCFG_R {
        CWCKCFG_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 4 - Clear codec not ready flag"]
    #[inline(always)]
    pub fn ccnrdy(&self) -> CCNRDY_R {
        CCNRDY_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Clear anticipated frame synchronization detection flag"]
    #[inline(always)]
    pub fn cafsdet(&self) -> CAFSDET_R {
        CAFSDET_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Clear late frame synchronization detection flag"]
    #[inline(always)]
    pub fn clfsdet(&self) -> CLFSDET_R {
        CLFSDET_R::new(((self.bits >> 6) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Clear overrun / underrun"]
    #[inline(always)]
    #[must_use]
    pub fn covrudr(&mut self) -> COVRUDR_W<0> {
        COVRUDR_W::new(self)
    }
    #[doc = "Bit 1 - Mute detection flag"]
    #[inline(always)]
    #[must_use]
    pub fn cmutedet(&mut self) -> CMUTEDET_W<1> {
        CMUTEDET_W::new(self)
    }
    #[doc = "Bit 2 - Clear wrong clock configuration flag"]
    #[inline(always)]
    #[must_use]
    pub fn cwckcfg(&mut self) -> CWCKCFG_W<2> {
        CWCKCFG_W::new(self)
    }
    #[doc = "Bit 4 - Clear codec not ready flag"]
    #[inline(always)]
    #[must_use]
    pub fn ccnrdy(&mut self) -> CCNRDY_W<4> {
        CCNRDY_W::new(self)
    }
    #[doc = "Bit 5 - Clear anticipated frame synchronization detection flag"]
    #[inline(always)]
    #[must_use]
    pub fn cafsdet(&mut self) -> CAFSDET_W<5> {
        CAFSDET_W::new(self)
    }
    #[doc = "Bit 6 - Clear late frame synchronization detection flag"]
    #[inline(always)]
    #[must_use]
    pub fn clfsdet(&mut self) -> CLFSDET_W<6> {
        CLFSDET_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SAI AClear flag register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [clrfr](index.html) module"]
pub struct CLRFR_SPEC;
impl crate::RegisterSpec for CLRFR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [clrfr::R](R) reader structure"]
impl crate::Readable for CLRFR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [clrfr::W](W) writer structure"]
impl crate::Writable for CLRFR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CLRFR to value 0"]
impl crate::Resettable for CLRFR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
