#[doc = "Register `DDRCTRL_INIT5` reader"]
pub struct R(crate::R<DDRCTRL_INIT5_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DDRCTRL_INIT5_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DDRCTRL_INIT5_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DDRCTRL_INIT5_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DDRCTRL_INIT5` writer"]
pub struct W(crate::W<DDRCTRL_INIT5_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DDRCTRL_INIT5_SPEC>;
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
impl From<crate::W<DDRCTRL_INIT5_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DDRCTRL_INIT5_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `MAX_AUTO_INIT_X1024` reader - MAX_AUTO_INIT_X1024"]
pub type MAX_AUTO_INIT_X1024_R = crate::FieldReader<u16, u16>;
#[doc = "Field `MAX_AUTO_INIT_X1024` writer - MAX_AUTO_INIT_X1024"]
pub type MAX_AUTO_INIT_X1024_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, DDRCTRL_INIT5_SPEC, u16, u16, 10, O>;
#[doc = "Field `DEV_ZQINIT_X32` reader - DEV_ZQINIT_X32"]
pub type DEV_ZQINIT_X32_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DEV_ZQINIT_X32` writer - DEV_ZQINIT_X32"]
pub type DEV_ZQINIT_X32_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, DDRCTRL_INIT5_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bits 0:9 - MAX_AUTO_INIT_X1024"]
    #[inline(always)]
    pub fn max_auto_init_x1024(&self) -> MAX_AUTO_INIT_X1024_R {
        MAX_AUTO_INIT_X1024_R::new((self.bits & 0x03ff) as u16)
    }
    #[doc = "Bits 16:23 - DEV_ZQINIT_X32"]
    #[inline(always)]
    pub fn dev_zqinit_x32(&self) -> DEV_ZQINIT_X32_R {
        DEV_ZQINIT_X32_R::new(((self.bits >> 16) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:9 - MAX_AUTO_INIT_X1024"]
    #[inline(always)]
    #[must_use]
    pub fn max_auto_init_x1024(&mut self) -> MAX_AUTO_INIT_X1024_W<0> {
        MAX_AUTO_INIT_X1024_W::new(self)
    }
    #[doc = "Bits 16:23 - DEV_ZQINIT_X32"]
    #[inline(always)]
    #[must_use]
    pub fn dev_zqinit_x32(&mut self) -> DEV_ZQINIT_X32_W<16> {
        DEV_ZQINIT_X32_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DDRCTRL SDRAM initialization register 5\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ddrctrl_init5](index.html) module"]
pub struct DDRCTRL_INIT5_SPEC;
impl crate::RegisterSpec for DDRCTRL_INIT5_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ddrctrl_init5::R](R) reader structure"]
impl crate::Readable for DDRCTRL_INIT5_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ddrctrl_init5::W](W) writer structure"]
impl crate::Writable for DDRCTRL_INIT5_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DDRCTRL_INIT5 to value 0x0010_0004"]
impl crate::Resettable for DDRCTRL_INIT5_SPEC {
    const RESET_VALUE: Self::Ux = 0x0010_0004;
}
