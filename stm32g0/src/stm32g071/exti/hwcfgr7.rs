#[doc = "Register `HWCFGR7` reader"]
pub struct R(crate::R<HWCFGR7_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<HWCFGR7_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<HWCFGR7_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<HWCFGR7_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `HWCFGR7` writer"]
pub struct W(crate::W<HWCFGR7_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<HWCFGR7_SPEC>;
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
impl From<crate::W<HWCFGR7_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<HWCFGR7_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CPUEVENT` reader - HW configuration CPU event generation"]
pub type CPUEVENT_R = crate::FieldReader<u32, u32>;
#[doc = "Field `CPUEVENT` writer - HW configuration CPU event generation"]
pub type CPUEVENT_W<'a, const O: u8> = crate::FieldWriter<'a, u32, HWCFGR7_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - HW configuration CPU event generation"]
    #[inline(always)]
    pub fn cpuevent(&self) -> CPUEVENT_R {
        CPUEVENT_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - HW configuration CPU event generation"]
    #[inline(always)]
    #[must_use]
    pub fn cpuevent(&mut self) -> CPUEVENT_W<0> {
        CPUEVENT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Hardware configuration registers\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hwcfgr7](index.html) module"]
pub struct HWCFGR7_SPEC;
impl crate::RegisterSpec for HWCFGR7_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [hwcfgr7::R](R) reader structure"]
impl crate::Readable for HWCFGR7_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [hwcfgr7::W](W) writer structure"]
impl crate::Writable for HWCFGR7_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets HWCFGR7 to value 0"]
impl crate::Resettable for HWCFGR7_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
