#[doc = "Register `TAFCR` reader"]
pub struct R(crate::R<TAFCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TAFCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TAFCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TAFCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TAFCR` writer"]
pub struct W(crate::W<TAFCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TAFCR_SPEC>;
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
impl From<crate::W<TAFCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TAFCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TAMP1E` reader - Tamper 1 detection enable"]
pub type TAMP1E_R = crate::BitReader<bool>;
#[doc = "Field `TAMP1E` writer - Tamper 1 detection enable"]
pub type TAMP1E_W<'a, const O: u8> = crate::BitWriter<'a, u32, TAFCR_SPEC, bool, O>;
#[doc = "Field `TAMP1TRG` reader - Active level for tamper 1"]
pub type TAMP1TRG_R = crate::BitReader<bool>;
#[doc = "Field `TAMP1TRG` writer - Active level for tamper 1"]
pub type TAMP1TRG_W<'a, const O: u8> = crate::BitWriter<'a, u32, TAFCR_SPEC, bool, O>;
#[doc = "Field `TAMPIE` reader - Tamper interrupt enable"]
pub type TAMPIE_R = crate::BitReader<bool>;
#[doc = "Field `TAMPIE` writer - Tamper interrupt enable"]
pub type TAMPIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, TAFCR_SPEC, bool, O>;
#[doc = "Field `TAMP1INSEL` reader - TAMPER1 mapping"]
pub type TAMP1INSEL_R = crate::BitReader<bool>;
#[doc = "Field `TAMP1INSEL` writer - TAMPER1 mapping"]
pub type TAMP1INSEL_W<'a, const O: u8> = crate::BitWriter<'a, u32, TAFCR_SPEC, bool, O>;
#[doc = "Field `TSINSEL` reader - TIMESTAMP mapping"]
pub type TSINSEL_R = crate::BitReader<bool>;
#[doc = "Field `TSINSEL` writer - TIMESTAMP mapping"]
pub type TSINSEL_W<'a, const O: u8> = crate::BitWriter<'a, u32, TAFCR_SPEC, bool, O>;
#[doc = "Field `ALARMOUTTYPE` reader - AFO_ALARM output type"]
pub type ALARMOUTTYPE_R = crate::BitReader<bool>;
#[doc = "Field `ALARMOUTTYPE` writer - AFO_ALARM output type"]
pub type ALARMOUTTYPE_W<'a, const O: u8> = crate::BitWriter<'a, u32, TAFCR_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - Tamper 1 detection enable"]
    #[inline(always)]
    pub fn tamp1e(&self) -> TAMP1E_R {
        TAMP1E_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Active level for tamper 1"]
    #[inline(always)]
    pub fn tamp1trg(&self) -> TAMP1TRG_R {
        TAMP1TRG_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Tamper interrupt enable"]
    #[inline(always)]
    pub fn tampie(&self) -> TAMPIE_R {
        TAMPIE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 16 - TAMPER1 mapping"]
    #[inline(always)]
    pub fn tamp1insel(&self) -> TAMP1INSEL_R {
        TAMP1INSEL_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - TIMESTAMP mapping"]
    #[inline(always)]
    pub fn tsinsel(&self) -> TSINSEL_R {
        TSINSEL_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - AFO_ALARM output type"]
    #[inline(always)]
    pub fn alarmouttype(&self) -> ALARMOUTTYPE_R {
        ALARMOUTTYPE_R::new(((self.bits >> 18) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Tamper 1 detection enable"]
    #[inline(always)]
    #[must_use]
    pub fn tamp1e(&mut self) -> TAMP1E_W<0> {
        TAMP1E_W::new(self)
    }
    #[doc = "Bit 1 - Active level for tamper 1"]
    #[inline(always)]
    #[must_use]
    pub fn tamp1trg(&mut self) -> TAMP1TRG_W<1> {
        TAMP1TRG_W::new(self)
    }
    #[doc = "Bit 2 - Tamper interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn tampie(&mut self) -> TAMPIE_W<2> {
        TAMPIE_W::new(self)
    }
    #[doc = "Bit 16 - TAMPER1 mapping"]
    #[inline(always)]
    #[must_use]
    pub fn tamp1insel(&mut self) -> TAMP1INSEL_W<16> {
        TAMP1INSEL_W::new(self)
    }
    #[doc = "Bit 17 - TIMESTAMP mapping"]
    #[inline(always)]
    #[must_use]
    pub fn tsinsel(&mut self) -> TSINSEL_W<17> {
        TSINSEL_W::new(self)
    }
    #[doc = "Bit 18 - AFO_ALARM output type"]
    #[inline(always)]
    #[must_use]
    pub fn alarmouttype(&mut self) -> ALARMOUTTYPE_W<18> {
        ALARMOUTTYPE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "tamper and alternate function configuration register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tafcr](index.html) module"]
pub struct TAFCR_SPEC;
impl crate::RegisterSpec for TAFCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [tafcr::R](R) reader structure"]
impl crate::Readable for TAFCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [tafcr::W](W) writer structure"]
impl crate::Writable for TAFCR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TAFCR to value 0"]
impl crate::Resettable for TAFCR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
