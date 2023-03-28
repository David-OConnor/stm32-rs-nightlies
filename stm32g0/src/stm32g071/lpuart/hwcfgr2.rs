#[doc = "Register `HWCFGR2` reader"]
pub struct R(crate::R<HWCFGR2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<HWCFGR2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<HWCFGR2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<HWCFGR2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `HWCFGR2` writer"]
pub struct W(crate::W<HWCFGR2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<HWCFGR2_SPEC>;
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
impl From<crate::W<HWCFGR2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<HWCFGR2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CFG1` reader - LUART hardware configuration 1"]
pub type CFG1_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CFG1` writer - LUART hardware configuration 1"]
pub type CFG1_W<'a, const O: u8> = crate::FieldWriter<'a, u32, HWCFGR2_SPEC, u8, u8, 4, O>;
#[doc = "Field `CFG2` reader - LUART hardware configuration 2"]
pub type CFG2_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CFG2` writer - LUART hardware configuration 2"]
pub type CFG2_W<'a, const O: u8> = crate::FieldWriter<'a, u32, HWCFGR2_SPEC, u8, u8, 4, O>;
impl R {
    #[doc = "Bits 0:3 - LUART hardware configuration 1"]
    #[inline(always)]
    pub fn cfg1(&self) -> CFG1_R {
        CFG1_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - LUART hardware configuration 2"]
    #[inline(always)]
    pub fn cfg2(&self) -> CFG2_R {
        CFG2_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - LUART hardware configuration 1"]
    #[inline(always)]
    #[must_use]
    pub fn cfg1(&mut self) -> CFG1_W<0> {
        CFG1_W::new(self)
    }
    #[doc = "Bits 4:7 - LUART hardware configuration 2"]
    #[inline(always)]
    #[must_use]
    pub fn cfg2(&mut self) -> CFG2_W<4> {
        CFG2_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "LPUART Hardware Configuration register 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hwcfgr2](index.html) module"]
pub struct HWCFGR2_SPEC;
impl crate::RegisterSpec for HWCFGR2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [hwcfgr2::R](R) reader structure"]
impl crate::Readable for HWCFGR2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [hwcfgr2::W](W) writer structure"]
impl crate::Writable for HWCFGR2_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets HWCFGR2 to value 0x13"]
impl crate::Resettable for HWCFGR2_SPEC {
    const RESET_VALUE: Self::Ux = 0x13;
}
