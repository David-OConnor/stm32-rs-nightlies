#[doc = "Register `HWCFGR1` reader"]
pub struct R(crate::R<HWCFGR1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<HWCFGR1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<HWCFGR1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<HWCFGR1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `HWCFGR1` writer"]
pub struct W(crate::W<HWCFGR1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<HWCFGR1_SPEC>;
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
impl From<crate::W<HWCFGR1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<HWCFGR1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CFG1` reader - LUART hardware configuration 1"]
pub type CFG1_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CFG1` writer - LUART hardware configuration 1"]
pub type CFG1_W<'a, const O: u8> = crate::FieldWriter<'a, u32, HWCFGR1_SPEC, u8, u8, 4, O>;
#[doc = "Field `CFG2` reader - LUART hardware configuration 2"]
pub type CFG2_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CFG2` writer - LUART hardware configuration 2"]
pub type CFG2_W<'a, const O: u8> = crate::FieldWriter<'a, u32, HWCFGR1_SPEC, u8, u8, 4, O>;
#[doc = "Field `CFG3` reader - LUART hardware configuration 1"]
pub type CFG3_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CFG3` writer - LUART hardware configuration 1"]
pub type CFG3_W<'a, const O: u8> = crate::FieldWriter<'a, u32, HWCFGR1_SPEC, u8, u8, 4, O>;
#[doc = "Field `CFG4` reader - LUART hardware configuration 2"]
pub type CFG4_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CFG4` writer - LUART hardware configuration 2"]
pub type CFG4_W<'a, const O: u8> = crate::FieldWriter<'a, u32, HWCFGR1_SPEC, u8, u8, 4, O>;
#[doc = "Field `CFG5` reader - LUART hardware configuration 2"]
pub type CFG5_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CFG5` writer - LUART hardware configuration 2"]
pub type CFG5_W<'a, const O: u8> = crate::FieldWriter<'a, u32, HWCFGR1_SPEC, u8, u8, 4, O>;
#[doc = "Field `CFG6` reader - LUART hardware configuration 2"]
pub type CFG6_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CFG6` writer - LUART hardware configuration 2"]
pub type CFG6_W<'a, const O: u8> = crate::FieldWriter<'a, u32, HWCFGR1_SPEC, u8, u8, 4, O>;
#[doc = "Field `CFG7` reader - LUART hardware configuration 2"]
pub type CFG7_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CFG7` writer - LUART hardware configuration 2"]
pub type CFG7_W<'a, const O: u8> = crate::FieldWriter<'a, u32, HWCFGR1_SPEC, u8, u8, 4, O>;
#[doc = "Field `CFG8` reader - LUART hardware configuration 2"]
pub type CFG8_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CFG8` writer - LUART hardware configuration 2"]
pub type CFG8_W<'a, const O: u8> = crate::FieldWriter<'a, u32, HWCFGR1_SPEC, u8, u8, 4, O>;
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
    #[doc = "Bits 8:11 - LUART hardware configuration 1"]
    #[inline(always)]
    pub fn cfg3(&self) -> CFG3_R {
        CFG3_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - LUART hardware configuration 2"]
    #[inline(always)]
    pub fn cfg4(&self) -> CFG4_R {
        CFG4_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - LUART hardware configuration 2"]
    #[inline(always)]
    pub fn cfg5(&self) -> CFG5_R {
        CFG5_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:23 - LUART hardware configuration 2"]
    #[inline(always)]
    pub fn cfg6(&self) -> CFG6_R {
        CFG6_R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bits 24:27 - LUART hardware configuration 2"]
    #[inline(always)]
    pub fn cfg7(&self) -> CFG7_R {
        CFG7_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bits 28:31 - LUART hardware configuration 2"]
    #[inline(always)]
    pub fn cfg8(&self) -> CFG8_R {
        CFG8_R::new(((self.bits >> 28) & 0x0f) as u8)
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
    #[doc = "Bits 8:11 - LUART hardware configuration 1"]
    #[inline(always)]
    #[must_use]
    pub fn cfg3(&mut self) -> CFG3_W<8> {
        CFG3_W::new(self)
    }
    #[doc = "Bits 12:15 - LUART hardware configuration 2"]
    #[inline(always)]
    #[must_use]
    pub fn cfg4(&mut self) -> CFG4_W<12> {
        CFG4_W::new(self)
    }
    #[doc = "Bits 16:19 - LUART hardware configuration 2"]
    #[inline(always)]
    #[must_use]
    pub fn cfg5(&mut self) -> CFG5_W<16> {
        CFG5_W::new(self)
    }
    #[doc = "Bits 20:23 - LUART hardware configuration 2"]
    #[inline(always)]
    #[must_use]
    pub fn cfg6(&mut self) -> CFG6_W<20> {
        CFG6_W::new(self)
    }
    #[doc = "Bits 24:27 - LUART hardware configuration 2"]
    #[inline(always)]
    #[must_use]
    pub fn cfg7(&mut self) -> CFG7_W<24> {
        CFG7_W::new(self)
    }
    #[doc = "Bits 28:31 - LUART hardware configuration 2"]
    #[inline(always)]
    #[must_use]
    pub fn cfg8(&mut self) -> CFG8_W<28> {
        CFG8_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "LPUART Hardware Configuration register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hwcfgr1](index.html) module"]
pub struct HWCFGR1_SPEC;
impl crate::RegisterSpec for HWCFGR1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [hwcfgr1::R](R) reader structure"]
impl crate::Readable for HWCFGR1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [hwcfgr1::W](W) writer structure"]
impl crate::Writable for HWCFGR1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets HWCFGR1 to value 0x3110_0000"]
impl crate::Resettable for HWCFGR1_SPEC {
    const RESET_VALUE: Self::Ux = 0x3110_0000;
}
