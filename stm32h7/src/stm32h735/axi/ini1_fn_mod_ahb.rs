#[doc = "Register `INI1_FN_MOD_AHB` reader"]
pub struct R(crate::R<INI1_FN_MOD_AHB_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<INI1_FN_MOD_AHB_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<INI1_FN_MOD_AHB_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<INI1_FN_MOD_AHB_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `INI1_FN_MOD_AHB` writer"]
pub struct W(crate::W<INI1_FN_MOD_AHB_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<INI1_FN_MOD_AHB_SPEC>;
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
impl From<crate::W<INI1_FN_MOD_AHB_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<INI1_FN_MOD_AHB_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RD_INC_OVERRIDE` reader - Converts all AHB-Lite write transactions to a series of single beat AXI"]
pub type RD_INC_OVERRIDE_R = crate::BitReader<bool>;
#[doc = "Field `RD_INC_OVERRIDE` writer - Converts all AHB-Lite write transactions to a series of single beat AXI"]
pub type RD_INC_OVERRIDE_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, INI1_FN_MOD_AHB_SPEC, bool, O>;
#[doc = "Field `WR_INC_OVERRIDE` reader - Converts all AHB-Lite read transactions to a series of single beat AXI"]
pub type WR_INC_OVERRIDE_R = crate::BitReader<bool>;
#[doc = "Field `WR_INC_OVERRIDE` writer - Converts all AHB-Lite read transactions to a series of single beat AXI"]
pub type WR_INC_OVERRIDE_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, INI1_FN_MOD_AHB_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - Converts all AHB-Lite write transactions to a series of single beat AXI"]
    #[inline(always)]
    pub fn rd_inc_override(&self) -> RD_INC_OVERRIDE_R {
        RD_INC_OVERRIDE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Converts all AHB-Lite read transactions to a series of single beat AXI"]
    #[inline(always)]
    pub fn wr_inc_override(&self) -> WR_INC_OVERRIDE_R {
        WR_INC_OVERRIDE_R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Converts all AHB-Lite write transactions to a series of single beat AXI"]
    #[inline(always)]
    #[must_use]
    pub fn rd_inc_override(&mut self) -> RD_INC_OVERRIDE_W<0> {
        RD_INC_OVERRIDE_W::new(self)
    }
    #[doc = "Bit 1 - Converts all AHB-Lite read transactions to a series of single beat AXI"]
    #[inline(always)]
    #[must_use]
    pub fn wr_inc_override(&mut self) -> WR_INC_OVERRIDE_W<1> {
        WR_INC_OVERRIDE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "AXI interconnect - INI x AHB functionality modification register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ini1_fn_mod_ahb](index.html) module"]
pub struct INI1_FN_MOD_AHB_SPEC;
impl crate::RegisterSpec for INI1_FN_MOD_AHB_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ini1_fn_mod_ahb::R](R) reader structure"]
impl crate::Readable for INI1_FN_MOD_AHB_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ini1_fn_mod_ahb::W](W) writer structure"]
impl crate::Writable for INI1_FN_MOD_AHB_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets INI1_FN_MOD_AHB to value 0x04"]
impl crate::Resettable for INI1_FN_MOD_AHB_SPEC {
    const RESET_VALUE: Self::Ux = 0x04;
}
