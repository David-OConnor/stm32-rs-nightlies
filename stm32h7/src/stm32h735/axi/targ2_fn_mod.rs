#[doc = "Register `TARG2_FN_MOD` reader"]
pub struct R(crate::R<TARG2_FN_MOD_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TARG2_FN_MOD_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TARG2_FN_MOD_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TARG2_FN_MOD_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TARG2_FN_MOD` writer"]
pub struct W(crate::W<TARG2_FN_MOD_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TARG2_FN_MOD_SPEC>;
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
impl From<crate::W<TARG2_FN_MOD_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TARG2_FN_MOD_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `READ_ISS_OVERRIDE` reader - Override AMIB read issuing capability"]
pub type READ_ISS_OVERRIDE_R = crate::BitReader<bool>;
#[doc = "Field `READ_ISS_OVERRIDE` writer - Override AMIB read issuing capability"]
pub type READ_ISS_OVERRIDE_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, TARG2_FN_MOD_SPEC, bool, O>;
#[doc = "Field `WRITE_ISS_OVERRIDE` reader - Override AMIB write issuing capability"]
pub type WRITE_ISS_OVERRIDE_R = crate::BitReader<bool>;
#[doc = "Field `WRITE_ISS_OVERRIDE` writer - Override AMIB write issuing capability"]
pub type WRITE_ISS_OVERRIDE_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, TARG2_FN_MOD_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - Override AMIB read issuing capability"]
    #[inline(always)]
    pub fn read_iss_override(&self) -> READ_ISS_OVERRIDE_R {
        READ_ISS_OVERRIDE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Override AMIB write issuing capability"]
    #[inline(always)]
    pub fn write_iss_override(&self) -> WRITE_ISS_OVERRIDE_R {
        WRITE_ISS_OVERRIDE_R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Override AMIB read issuing capability"]
    #[inline(always)]
    #[must_use]
    pub fn read_iss_override(&mut self) -> READ_ISS_OVERRIDE_W<0> {
        READ_ISS_OVERRIDE_W::new(self)
    }
    #[doc = "Bit 1 - Override AMIB write issuing capability"]
    #[inline(always)]
    #[must_use]
    pub fn write_iss_override(&mut self) -> WRITE_ISS_OVERRIDE_W<1> {
        WRITE_ISS_OVERRIDE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "AXI interconnect - TARG x long burst functionality modification\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [targ2_fn_mod](index.html) module"]
pub struct TARG2_FN_MOD_SPEC;
impl crate::RegisterSpec for TARG2_FN_MOD_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [targ2_fn_mod::R](R) reader structure"]
impl crate::Readable for TARG2_FN_MOD_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [targ2_fn_mod::W](W) writer structure"]
impl crate::Writable for TARG2_FN_MOD_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TARG2_FN_MOD to value 0x04"]
impl crate::Resettable for TARG2_FN_MOD_SPEC {
    const RESET_VALUE: Self::Ux = 0x04;
}
