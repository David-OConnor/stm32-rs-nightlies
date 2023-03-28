#[doc = "Register `DBG_CR` reader"]
pub struct R(crate::R<DBG_CR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DBG_CR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DBG_CR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DBG_CR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DBG_CR` writer"]
pub struct W(crate::W<DBG_CR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DBG_CR_SPEC>;
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
impl From<crate::W<DBG_CR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DBG_CR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DBG_STOP` reader - Debug Stop mode"]
pub type DBG_STOP_R = crate::BitReader<bool>;
#[doc = "Field `DBG_STOP` writer - Debug Stop mode"]
pub type DBG_STOP_W<'a, const O: u8> = crate::BitWriter<'a, u32, DBG_CR_SPEC, bool, O>;
#[doc = "Field `DBG_STANDBY` reader - Debug Standby and Shutdown modes"]
pub type DBG_STANDBY_R = crate::BitReader<bool>;
#[doc = "Field `DBG_STANDBY` writer - Debug Standby and Shutdown modes"]
pub type DBG_STANDBY_W<'a, const O: u8> = crate::BitWriter<'a, u32, DBG_CR_SPEC, bool, O>;
impl R {
    #[doc = "Bit 1 - Debug Stop mode"]
    #[inline(always)]
    pub fn dbg_stop(&self) -> DBG_STOP_R {
        DBG_STOP_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Debug Standby and Shutdown modes"]
    #[inline(always)]
    pub fn dbg_standby(&self) -> DBG_STANDBY_R {
        DBG_STANDBY_R::new(((self.bits >> 2) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - Debug Stop mode"]
    #[inline(always)]
    #[must_use]
    pub fn dbg_stop(&mut self) -> DBG_STOP_W<1> {
        DBG_STOP_W::new(self)
    }
    #[doc = "Bit 2 - Debug Standby and Shutdown modes"]
    #[inline(always)]
    #[must_use]
    pub fn dbg_standby(&mut self) -> DBG_STANDBY_W<2> {
        DBG_STANDBY_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DBG configuration register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dbg_cr](index.html) module"]
pub struct DBG_CR_SPEC;
impl crate::RegisterSpec for DBG_CR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dbg_cr::R](R) reader structure"]
impl crate::Readable for DBG_CR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dbg_cr::W](W) writer structure"]
impl crate::Writable for DBG_CR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DBG_CR to value 0"]
impl crate::Resettable for DBG_CR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
