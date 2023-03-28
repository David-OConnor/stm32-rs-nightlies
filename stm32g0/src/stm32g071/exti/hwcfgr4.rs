#[doc = "Register `HWCFGR4` reader"]
pub struct R(crate::R<HWCFGR4_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<HWCFGR4_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<HWCFGR4_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<HWCFGR4_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `HWCFGR4` writer"]
pub struct W(crate::W<HWCFGR4_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<HWCFGR4_SPEC>;
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
impl From<crate::W<HWCFGR4_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<HWCFGR4_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `EVENT_TRG` reader - HW configuration event trigger type"]
pub type EVENT_TRG_R = crate::FieldReader<u32, u32>;
#[doc = "Field `EVENT_TRG` writer - HW configuration event trigger type"]
pub type EVENT_TRG_W<'a, const O: u8> = crate::FieldWriter<'a, u32, HWCFGR4_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - HW configuration event trigger type"]
    #[inline(always)]
    pub fn event_trg(&self) -> EVENT_TRG_R {
        EVENT_TRG_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - HW configuration event trigger type"]
    #[inline(always)]
    #[must_use]
    pub fn event_trg(&mut self) -> EVENT_TRG_W<0> {
        EVENT_TRG_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Hardware configuration registers\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hwcfgr4](index.html) module"]
pub struct HWCFGR4_SPEC;
impl crate::RegisterSpec for HWCFGR4_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [hwcfgr4::R](R) reader structure"]
impl crate::Readable for HWCFGR4_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [hwcfgr4::W](W) writer structure"]
impl crate::Writable for HWCFGR4_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets HWCFGR4 to value 0"]
impl crate::Resettable for HWCFGR4_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
