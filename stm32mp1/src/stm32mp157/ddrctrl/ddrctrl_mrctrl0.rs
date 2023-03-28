#[doc = "Register `DDRCTRL_MRCTRL0` reader"]
pub struct R(crate::R<DDRCTRL_MRCTRL0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DDRCTRL_MRCTRL0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DDRCTRL_MRCTRL0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DDRCTRL_MRCTRL0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DDRCTRL_MRCTRL0` writer"]
pub struct W(crate::W<DDRCTRL_MRCTRL0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DDRCTRL_MRCTRL0_SPEC>;
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
impl From<crate::W<DDRCTRL_MRCTRL0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DDRCTRL_MRCTRL0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `MR_TYPE` reader - MR_TYPE"]
pub type MR_TYPE_R = crate::BitReader<bool>;
#[doc = "Field `MR_TYPE` writer - MR_TYPE"]
pub type MR_TYPE_W<'a, const O: u8> = crate::BitWriter<'a, u32, DDRCTRL_MRCTRL0_SPEC, bool, O>;
#[doc = "Field `MR_RANK` reader - MR_RANK"]
pub type MR_RANK_R = crate::BitReader<bool>;
#[doc = "Field `MR_RANK` writer - MR_RANK"]
pub type MR_RANK_W<'a, const O: u8> = crate::BitWriter<'a, u32, DDRCTRL_MRCTRL0_SPEC, bool, O>;
#[doc = "Field `MR_ADDR` reader - MR_ADDR"]
pub type MR_ADDR_R = crate::FieldReader<u8, u8>;
#[doc = "Field `MR_ADDR` writer - MR_ADDR"]
pub type MR_ADDR_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, DDRCTRL_MRCTRL0_SPEC, u8, u8, 4, O>;
#[doc = "Field `MR_WR` reader - MR_WR"]
pub type MR_WR_R = crate::BitReader<bool>;
#[doc = "Field `MR_WR` writer - MR_WR"]
pub type MR_WR_W<'a, const O: u8> = crate::BitWriter<'a, u32, DDRCTRL_MRCTRL0_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - MR_TYPE"]
    #[inline(always)]
    pub fn mr_type(&self) -> MR_TYPE_R {
        MR_TYPE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 4 - MR_RANK"]
    #[inline(always)]
    pub fn mr_rank(&self) -> MR_RANK_R {
        MR_RANK_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bits 12:15 - MR_ADDR"]
    #[inline(always)]
    pub fn mr_addr(&self) -> MR_ADDR_R {
        MR_ADDR_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bit 31 - MR_WR"]
    #[inline(always)]
    pub fn mr_wr(&self) -> MR_WR_R {
        MR_WR_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - MR_TYPE"]
    #[inline(always)]
    #[must_use]
    pub fn mr_type(&mut self) -> MR_TYPE_W<0> {
        MR_TYPE_W::new(self)
    }
    #[doc = "Bit 4 - MR_RANK"]
    #[inline(always)]
    #[must_use]
    pub fn mr_rank(&mut self) -> MR_RANK_W<4> {
        MR_RANK_W::new(self)
    }
    #[doc = "Bits 12:15 - MR_ADDR"]
    #[inline(always)]
    #[must_use]
    pub fn mr_addr(&mut self) -> MR_ADDR_W<12> {
        MR_ADDR_W::new(self)
    }
    #[doc = "Bit 31 - MR_WR"]
    #[inline(always)]
    #[must_use]
    pub fn mr_wr(&mut self) -> MR_WR_W<31> {
        MR_WR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Mode Register Read/Write Control Register 0. Do not enable more than one of the following fields simultaneously: sw_init_int pda_en mpr_en\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ddrctrl_mrctrl0](index.html) module"]
pub struct DDRCTRL_MRCTRL0_SPEC;
impl crate::RegisterSpec for DDRCTRL_MRCTRL0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ddrctrl_mrctrl0::R](R) reader structure"]
impl crate::Readable for DDRCTRL_MRCTRL0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ddrctrl_mrctrl0::W](W) writer structure"]
impl crate::Writable for DDRCTRL_MRCTRL0_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DDRCTRL_MRCTRL0 to value 0x10"]
impl crate::Resettable for DDRCTRL_MRCTRL0_SPEC {
    const RESET_VALUE: Self::Ux = 0x10;
}
