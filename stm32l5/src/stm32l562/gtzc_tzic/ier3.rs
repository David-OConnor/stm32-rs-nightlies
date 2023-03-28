#[doc = "Register `IER3` reader"]
pub struct R(crate::R<IER3_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<IER3_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<IER3_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<IER3_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `IER3` writer"]
pub struct W(crate::W<IER3_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<IER3_SPEC>;
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
impl From<crate::W<IER3_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<IER3_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TZSCIE` reader - TZSCIE"]
pub type TZSCIE_R = crate::BitReader<bool>;
#[doc = "Field `TZSCIE` writer - TZSCIE"]
pub type TZSCIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, IER3_SPEC, bool, O>;
#[doc = "Field `TZICIE` reader - TZICIE"]
pub type TZICIE_R = crate::BitReader<bool>;
#[doc = "Field `TZICIE` writer - TZICIE"]
pub type TZICIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, IER3_SPEC, bool, O>;
#[doc = "Field `MPCWM1IE` reader - MPCWM1IE"]
pub type MPCWM1IE_R = crate::BitReader<bool>;
#[doc = "Field `MPCWM1IE` writer - MPCWM1IE"]
pub type MPCWM1IE_W<'a, const O: u8> = crate::BitWriter<'a, u32, IER3_SPEC, bool, O>;
#[doc = "Field `MPCWM2IE` reader - MPCWM2IE"]
pub type MPCWM2IE_R = crate::BitReader<bool>;
#[doc = "Field `MPCWM2IE` writer - MPCWM2IE"]
pub type MPCWM2IE_W<'a, const O: u8> = crate::BitWriter<'a, u32, IER3_SPEC, bool, O>;
#[doc = "Field `MPCBB1IE` reader - MPCBB1IE"]
pub type MPCBB1IE_R = crate::BitReader<bool>;
#[doc = "Field `MPCBB1IE` writer - MPCBB1IE"]
pub type MPCBB1IE_W<'a, const O: u8> = crate::BitWriter<'a, u32, IER3_SPEC, bool, O>;
#[doc = "Field `MPCBB1_REGIE` reader - MPCBB1_REGIE"]
pub type MPCBB1_REGIE_R = crate::BitReader<bool>;
#[doc = "Field `MPCBB1_REGIE` writer - MPCBB1_REGIE"]
pub type MPCBB1_REGIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, IER3_SPEC, bool, O>;
#[doc = "Field `MPCBB2IE` reader - MPCBB2IE"]
pub type MPCBB2IE_R = crate::BitReader<bool>;
#[doc = "Field `MPCBB2IE` writer - MPCBB2IE"]
pub type MPCBB2IE_W<'a, const O: u8> = crate::BitWriter<'a, u32, IER3_SPEC, bool, O>;
#[doc = "Field `MPCBB2_REGIE` reader - MPCBB2_REGIE"]
pub type MPCBB2_REGIE_R = crate::BitReader<bool>;
#[doc = "Field `MPCBB2_REGIE` writer - MPCBB2_REGIE"]
pub type MPCBB2_REGIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, IER3_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - TZSCIE"]
    #[inline(always)]
    pub fn tzscie(&self) -> TZSCIE_R {
        TZSCIE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - TZICIE"]
    #[inline(always)]
    pub fn tzicie(&self) -> TZICIE_R {
        TZICIE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - MPCWM1IE"]
    #[inline(always)]
    pub fn mpcwm1ie(&self) -> MPCWM1IE_R {
        MPCWM1IE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - MPCWM2IE"]
    #[inline(always)]
    pub fn mpcwm2ie(&self) -> MPCWM2IE_R {
        MPCWM2IE_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - MPCBB1IE"]
    #[inline(always)]
    pub fn mpcbb1ie(&self) -> MPCBB1IE_R {
        MPCBB1IE_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - MPCBB1_REGIE"]
    #[inline(always)]
    pub fn mpcbb1_regie(&self) -> MPCBB1_REGIE_R {
        MPCBB1_REGIE_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - MPCBB2IE"]
    #[inline(always)]
    pub fn mpcbb2ie(&self) -> MPCBB2IE_R {
        MPCBB2IE_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - MPCBB2_REGIE"]
    #[inline(always)]
    pub fn mpcbb2_regie(&self) -> MPCBB2_REGIE_R {
        MPCBB2_REGIE_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - TZSCIE"]
    #[inline(always)]
    #[must_use]
    pub fn tzscie(&mut self) -> TZSCIE_W<0> {
        TZSCIE_W::new(self)
    }
    #[doc = "Bit 1 - TZICIE"]
    #[inline(always)]
    #[must_use]
    pub fn tzicie(&mut self) -> TZICIE_W<1> {
        TZICIE_W::new(self)
    }
    #[doc = "Bit 2 - MPCWM1IE"]
    #[inline(always)]
    #[must_use]
    pub fn mpcwm1ie(&mut self) -> MPCWM1IE_W<2> {
        MPCWM1IE_W::new(self)
    }
    #[doc = "Bit 3 - MPCWM2IE"]
    #[inline(always)]
    #[must_use]
    pub fn mpcwm2ie(&mut self) -> MPCWM2IE_W<3> {
        MPCWM2IE_W::new(self)
    }
    #[doc = "Bit 4 - MPCBB1IE"]
    #[inline(always)]
    #[must_use]
    pub fn mpcbb1ie(&mut self) -> MPCBB1IE_W<4> {
        MPCBB1IE_W::new(self)
    }
    #[doc = "Bit 5 - MPCBB1_REGIE"]
    #[inline(always)]
    #[must_use]
    pub fn mpcbb1_regie(&mut self) -> MPCBB1_REGIE_W<5> {
        MPCBB1_REGIE_W::new(self)
    }
    #[doc = "Bit 6 - MPCBB2IE"]
    #[inline(always)]
    #[must_use]
    pub fn mpcbb2ie(&mut self) -> MPCBB2IE_W<6> {
        MPCBB2IE_W::new(self)
    }
    #[doc = "Bit 7 - MPCBB2_REGIE"]
    #[inline(always)]
    #[must_use]
    pub fn mpcbb2_regie(&mut self) -> MPCBB2_REGIE_W<7> {
        MPCBB2_REGIE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "TZIC interrupt enable register 3\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ier3](index.html) module"]
pub struct IER3_SPEC;
impl crate::RegisterSpec for IER3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ier3::R](R) reader structure"]
impl crate::Readable for IER3_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ier3::W](W) writer structure"]
impl crate::Writable for IER3_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets IER3 to value 0"]
impl crate::Resettable for IER3_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
